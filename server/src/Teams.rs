use crate::{Channel::Channel, utils};

#[derive(Debug, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: String,
    pub channels: Vec<Channel>
}

impl Team {
    pub fn new() -> Team {
        Team { id: utils::generate_uuid(), name: String::new(), description: String::new(), channels: Vec::new() }
    }

    pub fn get_id(&self) -> String{
        self.id.clone()
    }

    pub fn set_name(&mut self, name: String) -> &mut Self{
        self.name = name;
        self
    }

    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }
}
