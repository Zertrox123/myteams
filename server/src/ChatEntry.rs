pub struct ChatEntry {
    pub id: String,
    pub auth_name: String,
    pub content: String,
    pub reply_id: Option<String>,
    pub emoji: Option<Vec<u32>>,
    pub timestamp: String,
}
