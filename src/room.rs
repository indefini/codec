use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type Rooms = HashMap<String, SyncRoom>;

pub type SyncRoom = Arc<RwLock<Room>>;

#[derive(Debug)]
pub struct Room
{
    id : String,
    pub name : String,
    pub topic : Option<String>,
    pub prev_batch : String,
    pub messages : Vec<Message>,
    pub users : HashMap<String, User>,
    //pub user_colors : HashMap<String, String>,
}

impl Room {
    pub fn new(id : &str, name : &str, prev_batch : &str) -> Room
    {
        Room {
            id : id.to_owned(),
            name : name.to_owned(),
            topic : None,
            prev_batch : prev_batch.to_owned(),
            messages : Vec::new(),
            users : HashMap::new()
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


#[derive(Debug)]
pub struct User
{
    pub id : String,
    pub display_name : Option<String>,
}

impl User
{
    pub fn new(id : String, display_name : Option<String>) -> User
    {
        User {
            id : id,
            display_name : display_name,
        }
    }

    pub fn get_name(&self) -> &str
    {
        if let Some(ref dn) = self.display_name {
            &*dn
        }
        else {
            &self.id
        }
    }
}

pub fn get_random_color() -> String
{
    "#ff0000".to_owned()
}
