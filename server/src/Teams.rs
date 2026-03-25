pub struct Team {
    pub id: String,
    pub name: String,
    pub description: String,
    pub channel: Vec<String>
}

impl Team {
    pub fn new() -> Team {
        Team { id: String::new(), name: String::new(), description: String::new(), channel: Vec::new() }
    }
}
