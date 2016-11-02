use std::collections::HashMap;
use serde::Serializer;

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

#[derive(Serialize, Deserialize, Debug)]
struct Event
{
    #[serde(serialize_with = "ser_content")]
    content : i32, //EventContent,
    origin_server_ts : Option<u64>,
    sender : Option<String>,
    #[serde(rename = "type")]
    kind : String,
    unsigned : Option<Unsigned>,
    state_key : Option<String>
}

fn ser_content<S>(t : &i32, serializer : &mut S) -> Result<(), S::Error> 
where S : Serializer
{
    serializer.serialize_i32(4)
}

#[derive(Serialize, Deserialize, Debug)]
struct Unsigned
{
    //prev_content : Option<EventContent>,
    age : Option<u64>,
    transaction_id : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
enum Membership
{
    Invite,
}

#[derive(Serialize, Deserialize, Debug)]
enum EventContent
{
    RoomMessage
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

