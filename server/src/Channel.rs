use crate::ChatEntry::ChatEntry;

pub struct Channel {
    pub id: String,
    pub name: String,
    pub messages: Vec<ChatEntry>
}
