use crate::{
    entities::{Channel, ChannelSchedule, Program},
    store::all_films,
};

pub struct Core {
    channels: Vec<Channel>,
}

impl Core {
    pub fn new() -> Self {
        let mut channels = Vec::new();

        // Genre
        channels.push(Channel::new("Full Clip"));
        channels.push(Channel::new("Midnight Run"));
        channels.push(Channel::new("Belly Laugh"));
        channels.push(Channel::new("That Feeling"));
        channels.push(Channel::new("The Magical Outdoors"));
        channels.push(Channel::new("Galactic Odyssey"));
        channels.push(Channel::new("The Dark Side"));

        // Subgenres
        channels.push(Channel::new("Across 110th Street"));
        channels.push(Channel::new("Hood dramas"));
        channels.push(Channel::new("Surreal & Satirical"));
        channels.push(Channel::new("Biopics"));
        channels.push(Channel::new("Tape"));
        channels.push(Channel::new("The Long Arc"));

        // Eras
        channels.push(Channel::new("70s"));
        channels.push(Channel::new("80s"));
        channels.push(Channel::new("90s"));
        channels.push(Channel::new("2000s"));
        channels.push(Channel::new("2010s"));
        channels.push(Channel::new("2020s"));
        channels.push(Channel::new("Our Time is Now"));

        // International
        channels.push(Channel::new("Diaspora"));
        channels.push(Channel::new("Diaspora, Nollywood"));

        // Regions
        channels.push(Channel::new("Southern Cinema"));
        channels.push(Channel::new("New York and the Northeast"));
        channels.push(Channel::new("West Side"));

        // Identity
        channels.push(Channel::new("Women"));
        channels.push(Channel::new("LGBTQ+"));

        // Non-fiction?
        channels.push(Channel::new("Documentaries"));
        channels.push(Channel::new("Stand-up Comedy"));

        Self { channels }
    }

    /// Gets the following information for each channel:
    /// - Channel object
    /// - Channel schedule for the given date
    /// - Currently playing program (if any)
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

    /// Builds a channel schedule for a given channel.
    pub fn build_channel_schedule(&self, _channel: &Channel) -> ChannelSchedule {
        let films = all_films();
        let mut schedule = ChannelSchedule::new();

        // From midnight until one minute before the end of the day, we loop through the films and add add them to the schedule as programs. If we reach the end of the film list, we start over from the beginning until we've filled the schedule for the entire day.
        let mut current_time = "00:00".to_string();

        // while current_time < String::from("23:59") {
        //     for film in &films {
        //         let program = Program {
        //             title: film.title.clone(),
        //             start_time: current_time.clone(),
        //             // Assume each programming block is 2 hours long for now, but we can adjust this later based on the actual runtime of the film
        //             end_time: {
        //                 let hours: u32 = current_time[0..2].parse().unwrap();
        //                 let minutes: u32 = current_time[3..5].parse().unwrap();
        //                 let total_minutes = hours * 60 + minutes + 120; // Add 2 hours
        //                 let new_hours = (total_minutes / 60) % 24; // Wrap around after 24 hours
        //                 let new_minutes = total_minutes % 60;
        //                 format!("{:02}:{:02}", new_hours, new_minutes)
        //             },
        //         };

        //         schedule.programming.push(program.clone());
        //         current_time = program.end_time.clone();
        //     }
        // }

        let all_day_program = Program {
            title: "All Day Programming".to_string(),
            start_time: "00:00".to_string(),
            end_time: "23:59".to_string(),
        };

        schedule.programming.push(all_day_program);

        schedule
    }

    /// Gets the currently playing program based on the current time and the channel schedule
    pub fn get_now_playing(&self, _schedule: &ChannelSchedule) -> Result<Program, String> {
        let current_time = "18:37".to_string();

        for program in &_schedule.programming {
            if current_time >= program.start_time && current_time <= program.end_time {
                return Ok(program.clone());
            }
        }

        Err("No program is currently playing.".to_string())
    }
}
