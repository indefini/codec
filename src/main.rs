extern crate hyper;
extern crate url;
extern crate rustc_serialize;
#[macro_use]
extern crate json;

use std::io::Read;
use hyper::{Client};
use url::form_urlencoded;
use rustc_serialize::{Encodable};
use json::{object,JsonValue};
use std::env;

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
const LOGIN : &'static str = "/_matrix/client/r0/login";
const GET_STATE : &'static str = "/_matrix/client/r0/sync?access_token=";//YOUR_ACCESS_TOKEN"
const GET_STATE_FILTER :&'static str = "/_matrix/client/r0/sync?filter={\"room\":{\"timeline\":{\"limit\":1}}}&access_token=";

//const SEND_MSG = &'static str = "_matrix/client/r0/rooms/%21asfLdzLnOdGRkdPZWu:localhost/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"

 //'{"msgtype":"m.text", "body":"hello"}' "https://localhost:8448/_matrix/client/r0/rooms/%21asfLdzLnOdGRkdPZWu:localhost/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"
 


fn main() {
    println!("Hello you!");
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough arguments");
    }

    let obj = object!{
        "type" => "m.login.password",
        "user" => args[1].as_str(),
        "password" => args[2].as_str()
    };

    let login_url = URL.to_owned() + LOGIN;
    let login =  post_json_object(&login_url, &obj).unwrap();

    println!("{}", login);

    let login = json::parse(&login).unwrap();


    let get_state_url = {
        let mut s = URL.to_owned() + GET_STATE_FILTER;
        if let Some(ref at) = login["access_token"].as_str() {
            s.push_str(at);
        }
        s
    };

    let state = get_content(&get_state_url).unwrap();
    println!("{}", state);

}

