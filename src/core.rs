use rand::prelude::*;

use crate::{
    entities::{Channel, ChannelSchedule, ChannelScheduleResponse, Film, Program},
    store::{all_channels, all_films},
};

pub struct Core {
    channels: Vec<Channel>,
    films: Vec<Film>,
}

impl Core {
    pub fn new() -> Self {
        let channels = all_channels();
        let films = all_films();

        Self { channels, films }
    }

    /// Gets the channel schedules for a given date.
    pub fn get_channel_schedules(&self, date: String) -> Vec<ChannelScheduleResponse> {
        self.channels
            .iter()
            .map(|channel| {
                let schedule = self.build_channel_schedule(date.clone(), channel);
                let now_playing = self.get_now_playing(&schedule);
                ChannelScheduleResponse {
                    channel: channel.clone(),
                    now_playing: now_playing.ok(),
                }
            })
            .collect()
    }

    /// Builds a channel schedule for a given channel.
    pub fn build_channel_schedule(&self, date: String, _channel: &Channel) -> ChannelSchedule {
        let day_of_week = match date.get(8..10) {
            Some(day_str) => day_str.parse::<u32>().unwrap_or(0) % 7,
            None => 0,
        };
        let mut rng = StdRng::seed_from_u64(day_of_week as u64);

        let mut schedule = ChannelSchedule::new();

        let mut current_time = 0; // Minutes since midnight
        let end_of_day = 24 * 60; // 1440 minutes in a day

        while current_time < end_of_day {
            let program_duration = 120; // Each program is 2 hours long (120 minutes)
            let end_time = current_time + program_duration;

            let film = match self.films.choose(&mut rng) {
                Some(film) => film,
                None => panic!("No films available to choose from."),
            };

            let program = Program {
                title: film.title.clone(),
                start_time: current_time,
                end_time: end_time,
            };

            schedule.programming.push(program);

            current_time = end_time
        }

        // schedule
        ChannelSchedule::new()
    }

    /// Gets the currently playing program based on the current time and the channel schedule
    pub fn get_now_playing(&self, schedule: &ChannelSchedule) -> Result<Program, String> {
        let current_time = 600; // Placeholder for current time in minutes since midnight (e.g., 6:00 AM)

        for program in &schedule.programming {
            if current_time >= program.start_time && current_time <= program.end_time {
                return Ok(program.clone());
            }
        }

        Err("No program is currently playing.".to_string())
    }
}
