use libc::{c_void, c_int, c_char, c_float};
use std::ffi::CStr;
use std::borrow::Cow;
use std::thread;
use std::sync::mpsc;
use std::sync::{RwLock, Arc, Mutex};
use chrono;
use chrono::TimeZone;
use std::time::Duration;
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;

use codec;
use matrix;
use efl;
use room;

use xdg;

const APP_NAME : &'static str = "codec";
const SESSION_NAME : &'static str = "session";

pub struct App
{
    core : Box<Core>,
}

impl App {
    pub fn new() -> App {
        let mut core = Box::new(Core::new());
        let ui_con = efl::UiCon::new(
            request_login_from_ui as *const c_void,
            key_press as *const c_void,
            &*core as *const _ as *const c_void);

         //for id in core.rooms.read().unwrap().keys() {
         for (id, r) in core.rooms.read().unwrap().iter() {
             ui_con.new_room(id);
             //tODO set title in new_room func
             let room = r.read().unwrap();
             let title = room.name.clone();// + room.topic.map_or("");
             ui_con.set_room_title(id, &title);
         }

        core.ui.lock().unwrap().con = Some(ui_con);
        core.ui.lock().unwrap().show_current();


        match (&core.session.user, &core.session.pass) {
            (&Some(ref u), &Some(ref p)) => {
                core.request_login(u, p);
            },
            _ => {}
        }

        App {
            core : core,
        }
    }

    pub fn save(&mut self) 
    {
        {
            let rooms = self.core.rooms.clone();
            let con = self.core.con.clone();
            let ui = self.core.ui.clone();

            let session = &mut self.core.session;
            for (id, room) in rooms.read().unwrap().iter() {
                //session.rooms.insert(id.clone(), room.read().unwrap().name.clone());
                session.rooms.insert(id.clone(), room.read().unwrap().clone());
            }

            session.next_batch = con.read().unwrap().next_batch.clone();

            session.current_room_id = ui.lock().unwrap().get_current_id().clone();
        }

        let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME).unwrap();
        let path = xdg_dirs.place_config_file(SESSION_NAME).expect("cannot create session file");

        let serialized = serde_json::to_string(&self.core.session).unwrap();
        //let path : &Path = Path::new("session");
        let mut file = File::create(path).ok().unwrap();
        file.write(serialized.as_bytes());
    }
}

type UiMasterMx = Arc<Mutex<UiMaster>>;
type Connection = Arc<RwLock<ConnectionData>>;
type Data = Arc<RwLock<room::Rooms>>;

struct UiMaster
{
    current : usize,
    rooms : Vec<String>,
    con : Option<efl::UiCon>
}

impl UiMaster {
    fn new() -> UiMaster
    {
        UiMaster {
            current : 0usize,
            rooms : Vec::new(),
            con : None
        }
    }

    fn get_current_id(&self) -> Option<String>
    {
        if self.rooms.is_empty() {
            None
        }
        else if self.current > self.rooms.len() -1 {
            None
        }
        else {
            Some(self.rooms[self.current].clone())
        }
    }

    fn show_next_room(&mut self)
    {
        //println!("yes ! next room  {}", self.current);
        if self.rooms.is_empty() {
            return;
        }

        self.current = if self.current >= self.rooms.len() - 1 {
            0
        }
        else {
            self.current + 1
        };

        //println!("yes ! current is now {} ", self.current);

        self.show_current();
    }

    fn set_room(&mut self, room_id : &str) -> bool
    {
        for (n, id) in self.rooms.iter().enumerate()
        {
            if id == room_id {
                self.current = n;
                return true;
            }
        }
        
        false
    }

    fn show_room(&mut self, room_id : &str)
    {
        if !self.set_room(room_id) {
            return;
        }

        self.show_current();
    }

    fn show_current(&self)
    {
        if self.rooms.is_empty() {
            return;
        }

        if let Some(ref con) = self.con {
            //println!("ok will try to show :{}, {}", self.current, self.rooms[self.current]);
            con.set_room(&self.rooms[self.current]);
        }

    }

}

struct Core
{
    rooms : Data,
    ui : UiMasterMx,
    con : Connection,
    session : codec::Session
}

#[derive(PartialEq)]
enum ConnectionState {
    Offline,
    Login,
    SyncFirst,
    SyncLoop
}

struct ConnectionData
{
    access_token : Option<String>,
    next_batch : Option<String>,
    state : ConnectionState
}

impl ConnectionData {
    fn new() -> ConnectionData {
        ConnectionData {
            access_token : None,
            next_batch : None,
            state : ConnectionState::Offline
        }
    }
}


impl Core
{
    pub fn new() -> Core
    {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME).unwrap();
        let path = xdg_dirs.place_config_file(SESSION_NAME).expect("cannot open session file");

        //let session : Session = match File::open(&Path::new("session")) {
        let session : codec::Session = match File::open(&path) {
            Ok(ref mut f) => {
                let mut file = String::new();
                f.read_to_string(&mut file).unwrap();
                serde_json::from_str(&file).unwrap()
            },
            _ => codec::Session::new()
        };

        let rooms : room::Rooms = session.rooms.iter().map(|(id, r)| (id.clone(), Arc::new(RwLock::new(r.clone())))).collect();


        let mut con = ConnectionData::new();
        con.next_batch = session.next_batch.clone();

        let mut ui = UiMaster::new();
        ui.rooms = session.rooms.keys().map(|s| s.clone()).collect();
        if let Some(ref id) = session.current_room_id {
            ui.set_room(id);
        }
            
        Core {
            ui : Arc::new(Mutex::new(ui)),
            con : Arc::new(RwLock::new(con)),
            rooms : Arc::new(RwLock::new(rooms)),
            session : session
        }
    }

    pub fn save_and_request_login(&mut self, user : &str, pass : &str)
    {
        self.session.user = Some(user.to_owned());
        self.session.pass = Some(pass.to_owned());
        self.request_login(user, pass);
    }

    pub fn request_login(&self, user : &str, pass : &str)
    {
        //let ui_con = self.ui_con.as_ref().unwrap();

        if let Some(ref ui_con) = self.ui.lock().unwrap().con {
            //efl::set_login_visible(false);
            ui_con.set_login_visible(false);
            ui_con.set_loading_visible(true);
            ui_con.clone()
        }
        else {
            return;
        };

        let users = user.to_owned();
        let passs = pass.to_owned();
        let (tx,rx) = mpsc::channel();


        /*
        efl::add_anim_fn(move || {
            //if let Ok(res) = self.log.try_lock()
            if let Ok(res) = rx.try_recv()
            {
            println!("done");
                false
            }
            else
            {
            println!("dance");
            true
            }
        });
        */

        self.con.write().unwrap().state = ConnectionState::Login;

        let child = thread::spawn(move || {
            let res = loginstring(users, passs);
            tx.send(res).unwrap();
        });

        let con_lock = self.con.clone();
        let rooms_lock = self.rooms.clone();
        let uimx = self.ui.clone();

        thread::spawn(move || {
            loop {
                if let Ok(login) = rx.try_recv() {

                    {
                        let mut con = con_lock.write().unwrap();
                        if con.next_batch.is_none() {
                            con.state = ConnectionState::SyncFirst;
                        }
                        else {
                            con.state = ConnectionState::SyncLoop;
                        }
                        con.access_token = Some(login.access_token.clone());
                    }

                    start_sync_task(uimx.clone(), con_lock, rooms_lock);
                    break;
                }
            }
        });

        //or show login failed + show the pass window again
        // 
    }

    pub fn handle_key(&mut self, modifier : &str, key : &str)
    {
        //println!("press key : {}, {}", modifier, key);

        if key == "Tab" {
            self.ui.lock().unwrap().show_next_room();
        }

    }
}

fn start_sync_task(uimx : UiMasterMx, con : Connection, rooms : Data)
{
    if con.read().unwrap().state == ConnectionState::SyncFirst {
        efl::main_loop_begin();
        if let Some(ref ui_con) = uimx.lock().unwrap().con {
            ui_con.set_loading_text("syncing");
        }
        efl::main_loop_end();
    }

    let (synctx,syncrx) = mpsc::channel();

    let access_token = con.read().unwrap().access_token.clone().unwrap();
    let access_token2 = access_token.clone();
    let con2 = con.clone();

    thread::spawn(move || {
        //println!("syncing started!!!");
        loop {
            let res = sync(&access_token2, con2.read().unwrap().next_batch.clone());
            if let Some(r) = res {
                synctx.send(r).unwrap();
            }

            let duration = Duration::from_secs(5);
            thread::sleep(duration);
        }
    });

    thread::spawn(move || {
        loop {
            if let Ok(sync) = syncrx.try_recv() {

                let mut co = con.write().unwrap();
                co.next_batch = Some(sync.next_batch.clone());

                if co.state == ConnectionState::SyncFirst {
                    //println!("syncing over!!!");
                    let r = get_rooms(&sync);
                    co.state = ConnectionState::SyncLoop;

                    efl::main_loop_begin();
                    //efl::add_async(|| {
                    //efl::set_loading_visible(false);
                    //efl::set_chat_visible(true);

                    if let Some(ref ui_con) = uimx.lock().unwrap().con {
                        ui_con.set_loading_visible(false);
                        ui_con.set_chat_visible(true);

                        for (id, room) in &r {
                            //println!("ok room : {}", room.read().unwrap().name);
                            if room.read().unwrap().name == TEST_ROOM {
                                add_chat_messages(&*ui_con, &*room.read().unwrap());
                            }
                        }
                    }

                    efl::main_loop_end();

                    *rooms.write().unwrap() = r;
                }
                else if co.state == ConnectionState::SyncLoop {
                    let m = get_new_messages(&sync);

                    let ui : &UiMaster = &*uimx.lock().unwrap();
                    if let Some(ref ui_con) = ui.con {

                        // just in case :: TODO remove
                        ui_con.set_loading_visible(false);
                        ui_con.set_chat_visible(true);
                    }

                    for (room_id, mut msg) in m {
                        if let Some(room) = rooms.read().unwrap().get(&*room_id) {
                            let mut rr = room.write().unwrap();
                            //if rr.name == TEST_ROOM {
                            //if rr.name == TEST_ROOM {
                            //if ui.current_room_id.map_or_else(|| false, |ref o| o.as_str() == room_id) {
                            //if ui.current_room_id.is_some() && *ui.current_room_id.as_ref().unwrap() == room_id {
                                add_messages_to_room(ui, &mut *rr, &mut msg);
                            //}
                            //else {
                             //   println!("got a message for room {}", rr.name);
                            //}
                        }
                    }
                }

            let duration = Duration::from_secs(5);
            thread::sleep(duration);
            }
        }
    });
}

fn get_rooms(sync : &Box<matrix::Sync>) -> room::Rooms
{
    let mut r = HashMap::new();
    for (id, room) in sync.rooms.join.iter() {

        let mut name = None;
        let mut messages = Vec::new();
        let mut topic = None;
        let mut users = HashMap::new();
        let mut creator = None;
        let mut federate = true;
        let mut join_rule = codec::JoinRule::NotSet;

        for e in room.state.events.iter() {
            match &*e.kind {
                "m.room.name" => {
                    if let Some(ref n) = e.content.name {
                        name = Some(n.clone());
                    }
                },
                "m.room.message" => {
                    if let Some(m) = get_message_from_event(e) {
                        messages.push(m);
                    }

                },
                "m.room.topic" => {
                    if let Some(ref t) = e.content.topic {
                        topic = Some(t.clone());
                    }

                },
                "m.room.member" => {
                    let sender = e.sender.clone().unwrap();
                    let user = codec::User::new(sender.clone(), e.content.displayname.clone());
                    users.insert(sender, user);
                },
                "m.room.third_party_invite" => {
                    println!(">>>TODO, third_party_invite ---");
                    //println!("{:?}", e);
                },
                "m.room.create" => {
                    creator = e.content.creator.clone();
                    federate = e.content.m_federate;
                },
                "m.room.aliases" => {
                    println!(">>>TODO, room aliases ---");
                },
                "m.room.join_rules" => {
                    join_rule = codec::get_join_rule(e.content.join_rule.as_ref().unwrap());
                },
                "m.room.power_levels" => {
                    println!(">>>TODO, room power levels ---");
                },
                "m.room.history_visibility" => {
                    println!(">>>TODO, room history visibility ---");
                },
                "m.room.canonical_alias" => {
                    if name.is_none() {
                        name = e.content.alias.clone()
                    }
                },
                _ => {
                    println!("______ get_rooms, TODo event : {} ", e.kind);
                }
            }
        }

        if name.is_none() {
            if !users.is_empty() {
                let mut members = String::new();
                for (id,u) in &users {
                    if !members.is_empty() {
                        members += ", ";
                    }
                    members += u.get_name();
                }

                name = Some(members);
            }
            else {
                //TODO
                name = Some("(no name), id :".to_owned() + id);
            }
        }

        let mut ro = codec::Room::new(
            id,
            &name.unwrap(),
            &room.timeline.prev_batch,
            creator.as_ref().unwrap(),
            federate,
            join_rule,
            );

        ro.messages = messages;
        ro.topic = topic;
        ro.users = users;

        r.insert(id.clone(), Arc::new(RwLock::new(ro)));
    }

    r
}

fn get_new_messages(sync : &Box<matrix::Sync>) -> HashMap<String, Vec<codec::Message>>
{
    let mut r = HashMap::new();
    for (id, room) in &sync.rooms.join {
        //println!("yes there is a room : {:?}", room);

        let mut messages = Vec::new();

        for e in room.timeline.events.iter() {
            //println!("yes there is an event : {:?}", e);
            match &*e.kind {
                "m.room.message" => {
                    //println!("yes there is a msg : {:?}", e.content);
                    if let Some(m) = get_message_from_event(e) {
                        messages.push(m);
                    }

                },
                _ => {
                    println!("TODO other event : >>>>>>>>>>>>>>>>>>>>>>");
                    println!("{:?}", e);
                    println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
                }
            }
        }

        //println!("id : {}, mmmmmmesss : {:?}", id, messages);

        r.insert(id.clone(), messages);
    }

    r
}

fn start_messages_task(
    uimx : UiMasterMx, 
    access_token : &str,
    room : room::SyncRoom)
{
    let (room_id, prev_batch, room_name) =
    {
        let room = room.read().unwrap();
        efl::main_loop_begin();
        if let Some(ref ui_con) = uimx.lock().unwrap().con {
            ui_con.set_loading_text(&("getting messages for".to_owned() + &room.name));
        }
        efl::main_loop_end();


        //let room_id = room.id().to_owned();
        //let prev_batch = room.prev_batch.clone();
        //let room_name = room.name.clone();
        //
        (room.id().to_owned(), room.prev_batch.clone(), room.name.clone())
    };

    let (tx, rx) = mpsc::channel();
    let access_token = access_token.to_owned();

    {
        let room_name = room_name.clone();

        thread::Builder::new().name(room_name.clone()).spawn(move || {
            //thread::spawn(move || {
            println!("get messages for {}", room_name);
            let res = get_room_messages(&access_token, &room_id, &prev_batch);
            tx.send(res).unwrap();
        });
    }

    thread::spawn(move || {
        loop {
            if let Ok(ref mut res) = rx.try_recv() {
                let mut room = room.write().unwrap();
                room.messages.append(res);
                println!("get messages for '{}' over!!!", room_name);
                efl::main_loop_begin();

                if let Some(ref ui_con) = uimx.lock().unwrap().con {
                    if room_name == TEST_ROOM {
                        ui_con.set_loading_visible(false);
                        ui_con.set_chat_visible(true);
                        //add_chat_messages(&*ui_con, &room.messages);
                        add_chat_messages(&*ui_con, &*room);
                    }
                }

                efl::main_loop_end();

                break;
            }
        }
    });

}

fn add_chat_messages(uicon : &efl::UiCon, room : &codec::Room)
{
    for m in &room.messages {
        match m.content {
            codec::Content::Text(ref t) => {
                let user = room.users.get(&m.user).unwrap().get_name();
                let name = "<color=#ff0000>".to_owned() + user + "</color>";
                uicon.add_room_text(&room.id, &name, &m.time, t);
            },
            _ => {
                println!("content is not text!!!!!!!!");
            }

        }
    }
}

fn add_messages_to_room(ui : &UiMaster, room : &mut codec::Room, messages : &mut Vec<codec::Message>)
{
    let uicon = if let Some(ref con) = ui.con {
        con
    }
    else {
        return;
    };

    efl::main_loop_begin();

    for m in &*messages {
        match m.content {
            codec::Content::Text(ref t) => {
                //let user = room.users.get(&m.user).unwrap().get_name();
                let user = room.users.get(&m.user).map_or("cannot find user", |u| u.get_name());
                let name = "<color=#ff0000>".to_owned() + user + "</color>";
                uicon.add_room_text(&room.id, &name, &m.time, t);
                if !ui.get_current_id().as_ref().map_or(false, |o| *o == room.id) {
                    uicon.notify(&room.name, &name, t);
                }
            },
            _ => {}
        }
    }

    efl::main_loop_end();

    room.messages.append(messages);
}


fn get_room_messages(
    access_token : &str,
    room_id : &str,
    prev_batch : &str) -> Vec<codec::Message>
{
    let msg_res = get_messages(
                access_token,
                room_id,
                prev_batch);

    //println!("MESSSSSSSSSSSSSSSSSS : {:?}", msg_res);

    let mut messages = Vec::new();

    for e in &msg_res.chunk {
        if let Some(m) = get_message_from_event(e) {
            messages.push(m);
        }
    }

    messages

    /*
    let room_messages : HashMap<String, Box<Messages>> = 
        sync.rooms.join.iter().map(|(id, room)|
            (id.clone(), get_messages(
                access_token,
                id,
                &room.timeline.prev_batch))).collect();

    for (room_id, messages) in &room_messages {
        let mut room = rooms.get_mut(room_id).unwrap();
        for e in &messages.chunk {
            if e.kind == "m.room.message" {
                let msgtype = if let Some(ref t) = e.content.msgtype {
                    t.clone()
                }
                else {
                    break;
                };

                let body = if let Some(ref body) = e.content.body {
                    body.clone()
                }
                else {
                        break
                };

                let m = match msgtype.as_str() {
                    "m.text" => room::Message::Text(body),
                    _ => break
                };

                room.messages.push(m);
            }
        }
    }

    rooms
    */
}

fn get_message_from_event(e : &matrix::Event) -> Option<codec::Message>
{
    if e.kind != "m.room.message" {
        return None;
    }

    let msgtype = if let Some(ref t) = e.content.msgtype {
        t.clone()
    }
    else {
        println!("_______no msgtype... : {:?}", e);
        return None;
    };

    let body = if let Some(ref body) = e.content.body {
        body.clone()
    }
    else {
        println!("__________no body");
        return None;
    };

    let c = match msgtype.as_str() {
        "m.text" => codec::Content::Text(body),
        _ => {
            println!("_____________ msgtype is not text : {}", msgtype );
            return None;
        }
    };

    let sender = if let Some(ref s) = e.sender {
        s.clone()
    }
    else {
        println!("_____________no sender");
        return None;
    };

    let time = if let Some(ost) = e.origin_server_ts {
        let today = chrono::offset::local::Local::today();
        let timezone = today.timezone();
        let date = timezone.timestamp(ost as i64/1000i64, 0u32);
        //date.to_rfc2822()
        date.time().format("%H:%M:%S").to_string()
    }
    else {
        println!("___________no timestamp");
        return None;
    };

    //println!("no problem... adding : {:?}", c);
    let m = codec::Message::new(&sender, &time, c);

    Some(m)
}


extern fn request_login_from_ui(
    data : *const c_void,
    user : *const c_char,
    pass : *const c_char)
{
    let core : *mut Core = data as *mut Core; 
    let core = unsafe { &mut *core };
    core.save_and_request_login(&*get_str(user), &*get_str(pass));  
}

extern fn key_press(
    data : *const c_void,
    modifier : *const c_char,
    key : *const c_char)
{
    let core : *mut Core = data as *mut Core; 
    let core = unsafe { &mut *core };
    core.handle_key(&*get_str(modifier), &*get_str(key));  
}


fn get_string(str : *const c_char) -> String {
    get_str(str).into_owned()
}

fn get_str<'a>(str : *const c_char) -> Cow<'a,str> {
    unsafe {
        CStr::from_ptr(str).to_string_lossy()
    }
}

use std::io::Read;
use hyper::{Client};
use url::form_urlencoded;
use rustc_serialize::{Encodable};
use json::{object,JsonValue};
use std::env;

use hyper;
use rustc_serialize;
use serde_json;
use json;

//#[cfg(feature = "serde_derive")]
//include!("serde_types.in.rs");

//#[cfg(feature = "serde_codegen")]
//include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

fn post_json<T>(url: &str, payload: &T) -> hyper::Result<String>
where T: Encodable {
    let client = Client::new();
    let body = rustc_serialize::json::encode(payload).unwrap();
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

fn post_json_object(url: &str, body: &JsonValue) -> hyper::Result<String>
{
    let client = Client::new();
    let mut response = try!(client.post(url).body(&body.dump()[..]).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

const POST_TEST : &'static str = "http://httpbin.org/post";


const URL : &'static str = "https://matrix.org:8448";
const PREFIX :&'static str = "/_matrix/client/r0";
const GET_STATE : &'static str = "/sync?access_token=";//YOUR_ACCESS_TOKEN"
const GET_STATE_FILTER :&'static str = "/sync?filter={\"room\":{\"timeline\":{\"limit\":1}}}&access_token=";
const GET_STATE_LOOP :&'static str = "/sync?access_token=";

//const SEND_MSG = &'static str = "_matrix/client/r0/rooms/%21asfLdzLnOdGRkdPZWu:localhost/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"

 //'{"msgtype":"m.text", "body":"hello"}' "https://localhost:8448/_matrix/client/r0/rooms/%21asfLdzLnOdGRkdPZWu:localhost/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"
 

const TEST_ROOM :&'static str = "christestroom";

fn loginstring(user : String, pass : String) -> Box<matrix::LoginResponse>
{
    login(user.as_str(), pass.as_str())
}

fn login(user : &str, pass : &str) -> Box<matrix::LoginResponse>
{
    let obj = object!{
        "type" => "m.login.password",
        "user" => user,
        "password" => pass
    };

    let login_url = URL.to_owned() + PREFIX + "/login";
    let login =  post_json_object(&login_url, &obj).unwrap();

    //println!("{}", login);

    //let login = json::parse(&login).unwrap();
    
    Box::new(serde_json::from_str(&login).unwrap())
}


 
fn sync(access_token : &str, next_batch : Option<String>) -> Option<Box<matrix::Sync>>
{
    let get_state_url = if let Some(ref nb) = next_batch {
        URL.to_owned() + PREFIX + GET_STATE + access_token + "&since=" + nb
    }
    else {
        URL.to_owned() + PREFIX + GET_STATE_FILTER + access_token
    };

    let state = match get_content(&get_state_url) {
        Ok(r) => r,
        Err(e) => {
            println!("error while syncing {}", e);
            return None;
        }
    };

    {
    let pretty = json::parse(&state);
    match pretty {
        Ok(o) =>  {
            //let state = pretty.pretty(2);
            //println!("{}", state);
        },
        Err(e) => {
            println!("error with json?? : {}", e);
            return None;
        }
    }
    }
    /*
    if let Some(ref next_batch) = state["next_batch"].as_str() {

    };

    for (key, value) in state["rooms"]["join"].entries() {
        println!("key : {}", key);
    }
    */

    Some(Box::new(serde_json::from_str(&state).unwrap()))

}

fn get_messages(access_token : &str, room_id : &str, from : &str) -> Box<matrix::Messages>
{
    let url = URL.to_owned() + PREFIX + "/rooms/" + room_id + "/messages" + "?from=" + from + "&dir=b&limit=10" + "&access_token=" + access_token;
    //println!("url : {}", url);
    let messages = get_content(&url).unwrap();

    //let pretty = json::parse(&messages).unwrap();
    //let ppp = pretty.pretty(2);
    //println!("{}", ppp);

    Box::new(serde_json::from_str(&messages).unwrap())
}



