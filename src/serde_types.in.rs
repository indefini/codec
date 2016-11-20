use std::collections::HashMap;
use serde::{Deserializer, Deserialize};
use serde::de::{Visitor, Error, MapVisitor};
use serde::de::impls::{IgnoredAny};
use std::path::Path;
use std::fs::File;

mod Matrix {
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
    pub highlight_count : u32,
    pub notification_count : u32
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EventContent
{
    pub body : Option<String>,
    pub msgtype : Option<String>,
    pub name : Option<String>,
    pub avatar_url : Option<String>,
    pub displayname : Option<String>,
    pub membership : Option<String>,
    pub topic : Option<String>
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


#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub user : Option<String>,
    pub pass : Option<String>,
    //pub next_batch : Option<String>,
//    pub rooms : HashMap<
}

impl Session {
    fn new() -> Session
    {
        Session {
            user : None,
            pass : None
        }
    }
}

