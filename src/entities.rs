#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
}

impl Channel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChannelSchedule {
    pub programming: Vec<Program>,
}

#[derive(Debug, Clone)]
pub struct ChannelScheduleResponse {
    pub now_playing: Option<Program>,
    pub programming: Vec<Program>,
    pub channel: Channel,
}

impl ChannelSchedule {
    pub fn new() -> Self {
        Self {
            programming: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub title: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone)]
pub struct Film {
    pub title: String,
}
