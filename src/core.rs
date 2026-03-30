use crate::entities::{Channel, ChannelSchedule, Program};

pub struct Core {
    channels: Vec<Channel>,
}

impl Core {
    pub fn new() -> Self {
        let mut channels = Vec::new();

        // Genre
        channels.push(Channel::new("Full Clip", "Action"));
        channels.push(Channel::new("Midnight Run", "Crime & Thrillers"));
        channels.push(Channel::new("Belly Laugh", "Comedy"));
        channels.push(Channel::new("That Feeling", "Romance"));
        channels.push(Channel::new("The Magical Outdoors", "Adventure & Fantasy"));
        channels.push(Channel::new("Galactic Odyssey", "Science Fiction"));
        channels.push(Channel::new("The Dark Side", "Horror"));

        // Subgenres
        channels.push(Channel::new("Across 110th Street", "Blaxploitation"));
        channels.push(Channel::new("Hood dramas", "Hood dramas"));
        channels.push(Channel::new("Surreal & Satirical", "Surreal & Satirical"));
        channels.push(Channel::new("Biopics", "Biopics"));
        channels.push(Channel::new("Tape", "Sports films"));
        channels.push(Channel::new("The Long Arc", "Civil Rights"));

        // Eras
        channels.push(Channel::new("70s", "70s"));
        channels.push(Channel::new("80s", "80s"));
        channels.push(Channel::new("90s", "90s"));
        channels.push(Channel::new("2000s", "2000s"));
        channels.push(Channel::new("2010s", "2010s"));
        channels.push(Channel::new("2020s", "2020s"));
        channels.push(Channel::new("Our Time is Now", "2020s"));

        // International
        channels.push(Channel::new("Diaspora", "Diaspora"));
        channels.push(Channel::new("Diaspora, Nollywood", "Diaspora, Nollywood"));

        // Regions
        channels.push(Channel::new("Southern Cinema", "Southern Cinema"));
        channels.push(Channel::new(
            "New York and the Northeast",
            "New York and the Northeast",
        ));
        channels.push(Channel::new("West Side", "LA, The Bay, and the West Coast"));

        // Identity
        channels.push(Channel::new("Women", "Women"));
        channels.push(Channel::new("LGBTQ+", "LGBTQ+"));

        // Non-fiction?
        channels.push(Channel::new("Documentaries", "Documentaries"));
        channels.push(Channel::new("Stand-up Comedy", "Stand-up Comedy"));

        Self { channels }
    }

    pub fn get_channel_schedules(
        &self,
        _date: &str,
        // XXX: String error :thumbsdown:
    ) -> Vec<(&Channel, ChannelSchedule, Result<Program, String>)> {
        self.channels
            .iter()
            .map(|channel| {
                let schedule = self.build_channel_schedule(channel);
                let now_playing = self.get_now_playing(&schedule);
                (channel, schedule, now_playing)
            })
            .collect()
    }

    pub fn build_channel_schedule(&self, _channel: &Channel) -> ChannelSchedule {
        let mut schedule = ChannelSchedule::new();

        let program = crate::entities::Program {
            title: "In Too Deeppppp".to_string(),
            description: "A cop goes undercover to infiltrate a drug operation, but finds himself in over his head.".to_string(),
            start_time: "12:00am".to_string(),
            end_time: "11:59pm".to_string(),
            runtime: "2h 0m".to_string(),
        };

        schedule.programming.push(program);

        schedule
    }

    pub fn get_now_playing(&self, _schedule: &ChannelSchedule) -> Result<Program, String> {
        let program = crate::entities::Program {
            title: "In Too DELETE MEEEEE".to_string(),
            description: "A cop goes undercover to infiltrate a drug operation, but finds himself in over his head.".to_string(),
            start_time: "12:00am".to_string(),
            end_time: "11:59pm".to_string(),
            runtime: "2h 0m".to_string(),
        };

        Ok(program)
    }
}
