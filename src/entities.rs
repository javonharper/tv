#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub description: String,
}

impl Channel {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Program {
    pub title: String,
    pub description: String,
    pub start_time: String,
    pub end_time: String,
    pub runtime: String,
}
#[derive(Debug, Clone)]
pub struct ChannelSchedule {
    pub programming: Vec<Program>,
}

impl ChannelSchedule {
    pub fn new() -> Self {
        Self {
            programming: Vec::new(),
        }
    }
}
