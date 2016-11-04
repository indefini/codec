use std::collections::HashMap;

pub type Rooms = HashMap<String, Room>;

#[derive(Debug)]
pub struct Room
{
    //id : String,
    name : String,
    messages : Vec<Message>
}

impl Room {
    pub fn new(name : &str) -> Room
    {
        Room {
            name : name.to_owned(),
            messages : Vec::new()
        }
    }
}

#[derive(Debug)]
pub enum Message
{
    Text(String),
    Image(String)
}

