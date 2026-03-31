use crate::{Channel::Channel, utils};

#[derive(Debug, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: String,
    pub channels: Vec<Channel>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            id: utils::generate_uuid(),
            name: String::new(),
            description: String::new(),
            channels: Vec::new(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn add_channel(&mut self, channel: Channel) -> Self {
        self.channels.push(channel);
        self.clone()
    }

    pub fn set_name(&mut self, name: String) -> Self {
        self.name = name;
        self.clone()
    }

    pub fn set_description(&mut self, description: String) -> Self {
        self.description = description;
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
            channels: Vec::new(),
        })
    }
}
