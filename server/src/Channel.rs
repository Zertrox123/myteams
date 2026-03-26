use std::collections::HashMap;

use crate::{ChatEntry::ChatEntry, utils};

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub threads: Vec<HashMap<String, Vec<ChatEntry>>>
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            id: utils::generate_uuid(),
            name: String::new(),
            description: String::new(),
            threads:Vec::new(),
        }
    }

    pub fn add_messages(&mut self,thread_id: String,  message: ChatEntry) -> &mut Self {
        for thread in self.threads.iter_mut() {
        }
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
