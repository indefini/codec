use std::collections::HashMap;
use serde::{Deserializer, Deserialize};
use serde::de::{Visitor, Error, MapVisitor};
use serde::de::impls::{IgnoredAny};
use std::path::Path;
use std::fs::File;

mod matrix {
use std::collections::HashMap;
    /// Login
#[derive(Serialize, Deserialize, Debug)]
    pub struct LoginResponse
    {
        pub access_token : String,
        pub home_server : String,
        pub user_id : String,
        pub refresh_token : Option<String>
    }

    /// Sync
#[derive(Serialize, Deserialize, Debug)]
    pub struct Sync {
        pub next_batch : String,
        pub rooms : Rooms,
        pub presence : Presence
    }

#[derive(Serialize, Deserialize, Debug)]
pub struct Rooms
{
    //leave : HashMap<String, LeftRoom>
    pub join : HashMap<String, JoinedRoom>
    //invite : HashMap<String, InvitedRoom>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinedRoom
{
    pub unread_notifications : UnreadNotificationCounts,
    pub timeline : Timeline,
    pub state : State,
    pub account_data : AccountData,
    pub ephemeral : Ephemeral
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnreadNotificationCounts
{
    pub highlight_count : Option<u32>,
    pub notification_count : Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeline
{
    pub limited : bool,
    pub prev_batch : String,
    pub events : Vec<Event>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State
{
    pub events : Vec<Event>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData
{
    pub events : Vec<Event>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ephemeral
{
    pub events : Vec<Event>
}

//struct InvitedRoom
//

#[derive(Serialize, Deserialize, Debug)]
pub struct Presence
{
    pub events : Vec<Event>
}

//#[derive(Serialize, Debug)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Event
{
    //#[serde(deserialize_with = "de_content")]
    pub content : EventContent,
    pub origin_server_ts : Option<u64>,
    pub sender : Option<String>,
    #[serde(rename = "type")]
    pub kind : String,
    pub unsigned : Option<Unsigned>,
    pub state_key : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unsigned
{
    pub prev_content : Option<EventContent>,
    pub age : Option<u64>,
    pub transaction_id : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Membership
{
    Invite,
}

fn return_true() -> bool
{
    true
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventContent
{
    pub body : Option<String>,
    pub msgtype : Option<String>,
    pub name : Option<String>,
    pub avatar_url : Option<String>,
    pub displayname : Option<String>,
    pub membership : Option<String>,
    pub topic : Option<String>,

    // third_party_invite
    pub display_name: Option<String>,
    pub key_validity_url: Option<String>,
    pub public_key : Option<String>,
    
    //room create
    pub creator : Option<String>,
    #[serde(rename = "m.federate", default = "return_true")]
    pub m_federate : bool,

    //room aliases
    pub aliases : Option<Vec<String>>,

    //room join rule
    pub join_rule : Option<String>,

    //room canonical alias
    pub alias : Option<String>

}

/*
#[derive(Serialize, Deserialize, Debug)]
struct RoomMessage {
    body : String,
    msgtype : String
}
*/

/// MESSAGES

#[derive(Serialize, Deserialize, Debug)]
pub struct Messages
{
    pub start : String,
    pub chunk : Vec<Event>,
    pub end : String
}

}


mod codec {
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub user : Option<String>,
    pub pass : Option<String>,
    pub next_batch : Option<String>,
    // id, name
    pub rooms : HashMap<String, Room>,
    pub current_room_id : Option<String>
}

impl Session {
    pub fn new() -> Session
    {
        Session {
            user : None,
            pass : None,
            rooms : HashMap::new(),
            next_batch : None,
            current_room_id : None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JoinRule
{
    NotSet,
    Public,
    Knock,
    Invite,
    Private
}

pub fn get_join_rule(s : &str) -> JoinRule
{
    match s {
        "public" => JoinRule::Public,
        "knock" => JoinRule::Knock,
        "invite" => JoinRule::Invite,
        "private" => JoinRule::Private,
        _ => {
            println!("join rules not supported : {}", s);
            JoinRule::NotSet
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room
{
    pub id : String,
    pub name : String,
    pub topic : Option<String>,
    pub prev_batch : String,
    //#[serde(skip_deserializing)]
    pub messages : Vec<Message>,
    pub users : HashMap<String, User>,
    pub creator : String,
    pub federate : bool,
    //pub user_colors : HashMap<String, String>,
    
    pub join_rule : JoinRule
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message
{
    pub user : String,
    pub time : String,
    pub content : Content
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content
{
    Text(String),
    Image(String)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User
{
    pub id : String,
    pub display_name : Option<String>,
}


}

