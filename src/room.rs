use std::collections::HashMap;

pub type Rooms = HashMap<String, Room>;

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

    pub fn id(&self) -> &str
    {
        &self.id
    }
}

#[derive(Debug)]
pub enum Message
{
    Text(String),
    Image(String)
}

