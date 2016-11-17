use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type Rooms = HashMap<String, SyncRoom>;

pub type SyncRoom = Arc<RwLock<Room>>;

#[derive(Debug)]
pub struct Room
{
    id : String,
    pub name : String,
    pub prev_batch : String,
    pub messages : Vec<Message>
}

impl Room {
    pub fn new(id : &str, name : &str, prev_batch : &str) -> Room
    {
        Room {
            id : id.to_owned(),
            name : name.to_owned(),
            prev_batch : prev_batch.to_owned(),
            messages : Vec::new()
        }
    }

    pub fn new_sync(id : &str, name : &str, prev_batch : &str) -> SyncRoom
    {
        Arc::new(RwLock::new(Room::new(id, name, prev_batch)))
    }

    pub fn id(&self) -> &str
    {
        &self.id
    }
}

#[derive(Debug)]
pub struct Message
{
    pub user : String,
    pub time : String,
    pub content : Content
}

impl Message {
    pub fn new(user : &str, time : &str, content : Content ) -> Message {
        Message {
            user : user.to_owned(),
            time : time.to_owned(),
            content : content
        }
    }
}


#[derive(Debug)]
pub enum Content
{
    Text(String),
    Image(String)
}

