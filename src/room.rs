use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use codec;

pub type Rooms = HashMap<String, SyncRoom>;

pub type SyncRoom = Arc<RwLock<codec::Room>>;

impl codec::Room {
    pub fn new(
        id : &str,
        name : &str,
        prev_batch : &str,
        creator : &str,
        federate : bool
        ) -> codec::Room
    {
        codec::Room {
            id : id.to_owned(),
            name : name.to_owned(),
            topic : None,
            prev_batch : prev_batch.to_owned(),
            messages : Vec::new(),
            users : HashMap::new(),
            creator : creator.to_owned(),
            federate : federate
        }
    }

    /*
    pub fn new_sync(id : &str, name : &str, prev_batch : &str) -> SyncRoom
    {
        Arc::new(RwLock::new(codec::Room::new(id, name, prev_batch)))
    }
    */

    pub fn id(&self) -> &str
    {
        &self.id
    }
}

impl codec::Message {
    pub fn new(user : &str, time : &str, content : codec::Content ) -> codec::Message {
        codec::Message {
            user : user.to_owned(),
            time : time.to_owned(),
            content : content
        }
    }
}

impl codec::User
{
    pub fn new(id : String, display_name : Option<String>) -> codec::User
    {
        codec::User {
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
