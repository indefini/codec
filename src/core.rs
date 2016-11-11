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
        //close the window,
        //show some loading icon
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
            let login = loginstring(users, passs);
            let res = sync(&login.access_token);
            if tx.send(res).is_err() {
                println!("could not send...");
            }
            //let mut log = mmm.lock().unwrap();
             // *log = Some(res);
        });

        thread::spawn(move || {
            loop {
                if let Ok(res) = rx.try_recv()
                {
                    efl::main_loop_begin();
                    //efl::add_async(|| {
                    //efl::set_loading_visible(false);
                    //efl::set_chat_visible(true);
                    
                    if let Ok(ui_con) = mu.lock() {
                    ui_con.set_loading_visible(false);
                    ui_con.set_chat_visible(true);
                    }
                    
                    //});
                    efl::main_loop_end();
                    break;
                }
                else
                {
                }
            }
        });

        /*
            let res = loginstring(users, passs);
            if tx.send(res).is_err() {
                println!("could not send... why");
            }
            */

        /*
        */

        //let res = child.join();
        //show "login success for 3sec"
        //show another text at the same time "syncing"
        //let sync = sync(&login.access_token);

        //ui_con.set_loading_visible(false);
        //ui_con.set_chat_visible(true);

        //or show login failed + show the pass window again
        // 
    }
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



