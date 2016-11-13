use room;
use efl;
use libc::{c_void, c_int, c_char, c_float};
use std::ffi::CStr;
use std::borrow::Cow;
use std::thread;
use std::sync::mpsc;
use std::sync::{RwLock, Arc, Mutex};

pub struct App
{
    core : Box<Core>,
}

impl App {
    pub fn new() -> App {
        let mut core = Box::new(Core::new());
        let ui_con = Arc::new(Mutex::new(efl::UiCon::new(
            request_login_from_ui as *const c_void,
            &*core as *const _ as *const c_void)));

        core.ui_con = Some(ui_con);

        App {
            core : core,
        }
    }
}

type UiCon = Arc<Mutex<efl::UiCon>>;

struct Core
{
    //access_token : String,
    //rooms : room::Rooms,
    ui_con : Option<UiCon>,
    //log : Arc<Mutex<Option<Box<LoginResponse>>>>,
    //tx : mpsc::Sender<Box<LoginResponse>>,
    //rx : mpsc::Receiver<Box<LoginResponse>>,

}

impl Core
{
    pub fn new() -> Core
    {
        //let (tx, rx) = mpsc::channel();

        Core {
            ui_con : None,
            //log : Arc::new(Mutex::new(None)),
            //tx : tx,
            //rx : rx
        }
    }

    pub fn request_login_from_ui(&self, user : &str, pass : &str)
    {
        //let ui_con = self.ui_con.as_ref().unwrap();

        let mu = if let Some(ref ui_con) = self.ui_con {
            if let Ok(ui_con) = ui_con.lock() {
            //efl::set_login_visible(false);
            ui_con.set_login_visible(false);
            ui_con.set_loading_visible(true);
            }
            ui_con.clone()
        }
        else {
            return;
        };

        //TODO
        //show "Login in" text
        
        let users = user.to_owned();
        let passs = pass.to_owned();
        //let mmm = self.log.clone();
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

        let child = thread::spawn(move || {
            let res = loginstring(users, passs);
            tx.send(res).unwrap();
        });

        thread::spawn(move || {
            loop {
                if let Ok(login) = rx.try_recv() {
                    start_sync_task(mu.clone(), login);
                    break;
                }
            }
        });

        //or show login failed + show the pass window again
        // 
    }
}

fn start_sync_task(mu : UiCon, login : Box<LoginResponse>)
{
    efl::main_loop_begin();
    if let Ok(ui_con) = mu.lock() {
        ui_con.set_loading_text("syncing");
    }
    efl::main_loop_end();

    let (synctx,syncrx) = mpsc::channel();

    thread::spawn(move || {
        println!("syncing started!!!");
        let res = sync(&login.access_token);
        synctx.send(res).unwrap();
    });

    thread::spawn(move || {
        loop {
            if let Ok(sync) = syncrx.try_recv() {
                println!("syncing over!!!");
                efl::main_loop_begin();
                //efl::add_async(|| {
                //efl::set_loading_visible(false);
                //efl::set_chat_visible(true);
                
                let mut rooms = get_rooms(&sync);

                if let Ok(ui_con) = mu.lock() {
                    ui_con.set_loading_visible(false);
                    ui_con.set_chat_visible(true);
                    ui_con.add_chat_text("chris", "14:00", "yoyo");
                    ui_con.add_chat_text("chris", "14:00", "yoyo");
                }

                efl::main_loop_end();
                

                break;
            }
        }
    });
}

fn get_rooms(sync : &Box<Sync>) -> room::Rooms
{
    let mut r = HashMap::new();
    for (id, room) in sync.rooms.join.iter() {

        let mut name = None;

        for e in room.state.events.iter() {
            if e.kind == "m.room.name" {
                if let Some(ref n) = e.content.name {
                    name = Some(n.clone());
                }
                break;
            }
        }

        if name.is_none() {
            name = Some("room has no name".to_owned());
        }

        r.insert(id.clone(), room::Room::new(&name.unwrap()));
    }

    r
}

fn get_rooms_info(access_token : &str, sync : &Box<Sync>) -> room::Rooms
{
    let mut rooms : room::Rooms =  { 
        let mut r = HashMap::new();
        for (id, room) in sync.rooms.join.iter() {

            let mut name = None;
            
            for e in room.state.events.iter() {
                if e.kind == "m.room.name" {
                    if let Some(ref n) = e.content.name {
                        name = Some(n.clone());
                    }
                    break;
                }
            }

            if name.is_none() {
                name = Some("room has no name".to_owned());
            }

            r.insert(id.clone(), room::Room::new(&name.unwrap()));
        }
        r
    };

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
}


extern fn request_login_from_ui(
    data : *const c_void,
    user : *const c_char,
    pass : *const c_char)
{
    let core : *const Core = data as *const Core; 
    let core = unsafe { &*core };
    core.request_login_from_ui(&*get_str(user), &*get_str(pass));  
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

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

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

//const SEND_MSG = &'static str = "_matrix/client/r0/rooms/%21asfLdzLnOdGRkdPZWu:localhost/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"

 //'{"msgtype":"m.text", "body":"hello"}' "https://localhost:8448/_matrix/client/r0/rooms/%21asfLdzLnOdGRkdPZWu:localhost/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"
 

fn loginstring(user : String, pass : String) -> Box<LoginResponse>
{
    login(user.as_str(), pass.as_str())
}

fn login(user : &str, pass : &str) -> Box<LoginResponse>
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


 
fn sync(access_token : &str) -> Box<Sync>
{
    let get_state_url = URL.to_owned() + PREFIX + GET_STATE_FILTER + access_token;

    let state = get_content(&get_state_url).unwrap();

    let pretty = json::parse(&state).unwrap();
    let state = pretty.pretty(2);
    println!("{}", state);
    /*
    if let Some(ref next_batch) = state["next_batch"].as_str() {

    };

    for (key, value) in state["rooms"]["join"].entries() {
        println!("key : {}", key);
    }
    */

    Box::new(serde_json::from_str(&state).unwrap())

}

fn get_messages(access_token : &str, room_id : &str, from : &str) -> Box<Messages>
{
    let url = URL.to_owned() + PREFIX + "/rooms/" + room_id + "/messages" + "?from=" + from + "&dir=b&limit=10" + "&access_token=" + access_token;
    //println!("url : {}", url);
    let messages = get_content(&url).unwrap();

    //let pretty = json::parse(&messages).unwrap();
    //let ppp = pretty.pretty(2);
    //println!("{}", ppp);

    Box::new(serde_json::from_str(&messages).unwrap())
}



