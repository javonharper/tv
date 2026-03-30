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

pub struct ChannelSchedule {
    // name: String,
    // description: String,
}

impl ChannelSchedule {
    pub fn new() -> Self {
        Self {
            // name: name.to_string(),
            // description: description.to_string(),
        }
    }
}
