#[derive(Debug, Clone)]
pub struct Channel {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ChannelSchedule {
    pub programming: Vec<Program>,
}

#[derive(Debug, Clone)]
pub struct ChannelScheduleResponse {
    pub now_playing: Option<Program>,
    pub next_up: Option<Program>,
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
    pub start_time: i32, // Minutes since midnight
    pub end_time: i32,   // Minutes since midnight
}

#[derive(Debug, Clone)]
pub struct Film {
    pub title: String,
    pub collections: Vec<String>,
}
