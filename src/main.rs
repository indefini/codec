#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate url;
extern crate rustc_serialize;
#[macro_use]
extern crate json;
extern crate rscam;

extern crate libc;

extern crate efl_sys as efl;

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

mod room;
mod core;

fn main() {
    //println!("Hello you!");
    /*
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough arguments");
    }

    let login = login(args[1].as_str(), args[2].as_str());
    let sync = sync(&login.access_token);

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
                &login.access_token,
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

    println!("rooms : {:?}", rooms);
    */

    efl::app_init();
    core::App::new();
    efl::app_run();

}

