use crate::{ChatEntry::ChatEntry, utils};

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub messages: Vec<ChatEntry>
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            id: utils::generate_uuid(),
            name: String::new(),
            description: String::new(),
            messages:Vec::new(),
        }
    }

    pub fn add_messages(&mut self, message: ChatEntry) -> &mut Self {
        self.messages.push(message);
        self
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    
}
