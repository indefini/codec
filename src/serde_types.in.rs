use std::collections::HashMap;
use serde::{Deserializer, Deserialize};
use serde::de::{Visitor, Error, MapVisitor};
use serde::de::impls::{IgnoredAny};

/// Login
#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse
{
    access_token : String,
    home_server : String,
    user_id : String,
    refresh_token : Option<String>
}

/// Sync
#[derive(Serialize, Deserialize, Debug)]
struct Sync {
    next_batch : String,
    rooms : Rooms,
    presence : Presence
}

#[derive(Serialize, Deserialize, Debug)]
struct Rooms
{
    //leave : HashMap<String, LeftRoom>
    join : HashMap<String, JoinedRoom>
    //invite : HashMap<String, InvitedRoom>
}

#[derive(Serialize, Deserialize, Debug)]
struct JoinedRoom
{
    unread_notifications : UnreadNotificationCounts,
    timeline : Timeline,
    state : State,
    account_data : AccountData,
    ephemeral : Ephemeral
}

#[derive(Serialize, Deserialize, Debug)]
struct UnreadNotificationCounts
{
    highlight_count : u32,
    notification_count : u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Timeline
{
    limited : bool,
    prev_batch : String,
    events : Vec<Event>
}

#[derive(Serialize, Deserialize, Debug)]
struct State
{
    events : Vec<Event>
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountData
{
    events : Vec<Event>
}

#[derive(Serialize, Deserialize, Debug)]
struct Ephemeral
{
    events : Vec<Event>
}

//struct InvitedRoom
//

#[derive(Serialize, Deserialize, Debug)]
struct Presence
{
    events : Vec<Event>
}

//#[derive(Serialize, Debug)]
#[derive(Serialize, Deserialize, Debug)]
struct Event
{
    //#[serde(deserialize_with = "de_content")]
    content : EventContent,
    origin_server_ts : Option<u64>,
    sender : Option<String>,
    #[serde(rename = "type")]
    kind : String,
    unsigned : Option<Unsigned>,
    state_key : Option<String>
}

/*
impl Deserialize for EventContent
{
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: Deserializer
    {
        enum Field { Body, MsgType };

        impl Deserialize for Field {
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error> where D: Deserializer
            {
                struct FieldVisitor;

                impl Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E> where E: Error,
                    {
                        match value {
                            "body" => Ok(Field::Body),
                            "msgtype" => Ok(Field::MsgType),
                            _ => Err(Error::unknown_field(value)),
                        }
                    }
                }

                deserializer.deserialize_struct_field(FieldVisitor)
            }
        }

        struct EventContentVisitor {
            kind : String
        }

        impl Visitor for EventContentVisitor {
            type Value = EventContent;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Event, V::Error> where V: MapVisitor
            {
                let mut body: Option<String> = None;
                let mut msgtype: Option<String> = None;

                while let Some(key) = try!(visitor.visit_key::<Field>()) {
                    match key {
                        Field::Body => {
                            body = Some(try!(visitor.visit_value()));
                        }
                        Field::MsgType => {
                            msgtype = Some(try!(visitor.visit_value()));
                        }
                    }
                }
                try!(visitor.end());

                match self.kind {
                    "m.room.message" => {
                        Ok(EventContent::RoomMessage::new(body.unwrap(), msgtype.unwrap()))
                    }
                    _ => {
                        Err!("I don't know this event yet");
                    }
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["body", "msgtype"];
        deserializer.deserialize_struct("EventContent", FIELDS, EventContentVisitor)
    }
}
*/


/*
impl Deserialize for Event
{
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: Deserializer
    {
        #[derive(Debug)]
        enum Field { Content, OriginServerTs, Sender, Kind, Unsigned, StateKey };

        impl Deserialize for Field {
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error> where D: Deserializer
            {
                struct FieldVisitor;

                impl Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E> where E: Error,
                    {
                        match value {
                            "content" => Ok(Field::Content),
                            "origin_server_ts" => Ok(Field::OriginServerTs),
                            "sender" => Ok(Field::Sender),
                            "type" => Ok(Field::Kind),
                            "unsigned" => Ok(Field::Unsigned),
                            "state_key" => Ok(Field::StateKey),
                            _ => Err(Error::unknown_field(value)),
                        }
                    }
                }

                deserializer.deserialize_struct_field(FieldVisitor)
            }
        }

        struct EventVisitor
        {
            kind : Option<String>
        }

        impl Visitor for EventVisitor {
            type Value = Event;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Event, V::Error> where V: MapVisitor
            {
                let mut origin_server_ts: Option<u64> = None;
                let mut sender: Option<String> = None;
                let mut kind: Option<String> = None;
                let mut unsigned: Option<Unsigned> = None;
                let mut state_key : Option<String> = None;

                println!("dance-----------------------");

                //while let Some(key) = try!(visitor.visit_key::<Field>()) {
                    //println!("key : {:?}", key);
                    //match key {
                        //Field::Content => {
                            //let ev : Option<EventContent> = Some(try!(visitor.visit_value()));
                        //},
                        //Field::OriginServerTs => {
                            //origin_server_ts = Some(try!(visitor.visit_value()));
                        //},
                        //Field::Sender => {
                            //sender = Some(try!(visitor.visit_value()));
                        //},
                        //Field::Kind => {
                            //kind = Some(try!(visitor.visit_value()));
                        //},
                        //Field::Unsigned => {
                            //unsigned = Some(try!(visitor.visit_value()));
                        //},
                        //Field::StateKey => {
                            //state_key = Some(try!(visitor.visit_value()));
                        //},
                    //}
                //}
                //

                while let Some(key) = try!(visitor.visit_key::<String>()) {
                    println!("key : {:?}", key);
                    match key.as_str() {
                        "content" => {
                            let ev : Result<IgnoredAny, V::Error> = visitor.visit_value();
                        },
                        "origin_server_ts" => {
                            origin_server_ts = Some(try!(visitor.visit_value()));
                        },
                        "server" => {
                            sender = Some(try!(visitor.visit_value()));
                        },
                        "type" => {
                            kind = Some(try!(visitor.visit_value()));
                        },
                        "unsigned" => {
                            unsigned = Some(try!(visitor.visit_value()));
                        },
                        "state_key" => {
                            state_key = Some(try!(visitor.visit_value()));
                        },
                        _ => {
                            let v : Result<IgnoredAny, V::Error> = visitor.visit_value();
                        }
                    }

                    println!("read the value of : {:?}", key);
                }

                println!("il plante avant ca hein ");

                try!(visitor.end());
                Ok(Event {
                    content : None,
                    origin_server_ts : origin_server_ts,
                    sender : sender,
                    kind : kind.unwrap(),
                    unsigned : unsigned,
                    state_key : state_key
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["type", "origin_server_ts", "sender", "unsigned", "state_key", "content"];
        let result = deserializer.deserialize_struct("Event", FIELDS, EventVisitor { kind : None} );

        if let Ok(r) = result {
        const FIELDS: &'static [&'static str] = &["origin_server_ts", "sender", "type", "unsigned", "state_key", "content"];
        deserializer.deserialize_struct("Event", FIELDS, EventVisitor{ kind : Some(r.kind) })
        }
        else {
            Err(serde::de::Error::custom("shit"))
        }
    }
}
*/





/*
fn de_content<D>(de : &mut D) -> Result<i32, D::Error> 
where D : Deserializer
{
    Ok(4)
}
*/

#[derive(Serialize, Deserialize, Debug)]
struct Unsigned
{
    prev_content : Option<EventContent>,
    age : Option<u64>,
    transaction_id : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
enum Membership
{
    Invite,
}

#[derive(Serialize, Deserialize, Debug)]
struct EventContent
{
    body : Option<String>,
    msgtype : Option<String>,
    name : Option<String>,
    avatar_url : Option<String>,
    displayname : Option<String>,
    membership : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RoomMessage {
    body : String,
    msgtype : String
}

/*
enum MessageBody
{
    Text(String),
    Notice(String),
    Emote(String),
    Image(Image)
}

struct Image {
    body :
}
*/

/// MESSAGES

#[derive(Serialize, Deserialize, Debug)]
struct Messages
{
    start : String,
    chunk : Vec<Event>,
    end : String
}

#[derive(Serialize, Deserialize, Debug)]
struct EmptyStruct;

