use crate::{ChatEntry::ChatEntry, utils};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Threads {
    pub id: String,
    pub name: String,
    pub message: String,
    pub reply: Vec<ChatEntry>,
}

impl Threads {
    pub fn to_string(&self) -> String {
        format!("{}\n{}\n{}", self.id, self.name, self.message)
    }

    pub fn from_string(info: Vec<&str>) -> Result<Self, String> {
        Ok(Self {
            id: info.get(0).unwrap().to_string(),
            name: info.get(1).unwrap().to_string(),
            message: info.get(2).unwrap().to_string(),
            reply: Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub threads: Vec<Threads>,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            id: utils::generate_uuid(),
            name: String::new(),
            description: String::new(),
            threads: Vec::new(),
        }
    }

    pub fn add_messages(&mut self, thread_id: String, message: ChatEntry) -> &mut Self {
        for thread in &mut self.threads{
            if thread.id == thread_id {
                thread.reply.push(message.clone());
            }
        }
        self
    }

    pub fn add_threads(&mut self, id: String, message: String, name: String) -> &mut Self {
        self.threads.push(Threads { id,
            name,
            message,
            reply: Vec::new()
        });
        self
    }
    pub fn set_description(&mut self, description: String) -> Self {
        self.description = description;
        self.clone()
    }

    pub fn set_name(&mut self, name: String) -> Self {
        self.name = name;
        self.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{}\n{}\n{}", self.id, self.name, self.description)
    }

    pub fn from_string(info: Vec<&str>) -> Result<Self, String> {
        Ok(Self {
            id: info.get(0).unwrap().to_string(),
            name: info.get(1).unwrap().to_string(),
            description: info.get(2).unwrap().to_string(),
            threads: Vec::new(),
        })
    }
}
