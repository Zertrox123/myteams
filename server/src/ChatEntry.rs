use crate::utils;

#[derive(Debug, Clone)]
pub struct ChatEntry {
    pub id: String,
    pub auth_name: String,
    pub content: String,
    pub reply_id: Option<String>,
    pub emoji: Option<Vec<u32>>,
    pub timestamp: String,
}

impl ChatEntry {
    pub fn new() -> ChatEntry {
        ChatEntry { id: utils::generate_uuid(),
                    auth_name: String::new(),
                    content: String::new(),
                    reply_id: None,
                    emoji: None,
                    timestamp: String::new(),
        }
    }
    
}
