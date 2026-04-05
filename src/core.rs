use rand::prelude::*;

static SEED: u64 = 10;

use crate::{
    entities::{Channel, ChannelSchedule, ChannelScheduleResponse, Film, Program},
    store::{all_channels, all_films},
    utils::current_time_in_minutes,
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
                let next_up = self.get_next_program(&schedule);
                ChannelScheduleResponse {
                    channel: channel.clone(),
                    now_playing: now_playing.ok(),
                    next_up: next_up.ok(),
                }
            })
            .collect()
    }

    /// Builds a channel schedule for a given channel.
    pub fn build_channel_schedule(&self, date: String, _channel: &Channel) -> ChannelSchedule {
        let day_of_week = match date.get(8..10) {
            Some(day_str) => day_str.parse::<u32>().unwrap_or(0) % 7,
            None => panic!("Invalid date format. Expected YYYY-MM-DD."),
        };
        let mut rng = StdRng::seed_from_u64(day_of_week as u64 + SEED);

        let mut schedule = ChannelSchedule::new();

        let mut current_time = 0; // Minutes since midnight
        let end_of_day = 24 * 60 - 1; // Last minute of the day

        while current_time < end_of_day {
            let program_duration = 120; // Each program is 2 hours long (120 minutes)
            let end_time = current_time + program_duration;
            let films = self
                .films
                .iter()
                .filter(|film| film.collections.contains(&_channel.key))
                .collect::<Vec<_>>();

            let film = match films.choose(&mut rng) {
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

        schedule
    }

    /// Gets the program playing at a specific time (in minutes since midnight)
    fn get_playing_at(
        &self,
        schedule: &ChannelSchedule,
        time_in_minutes: i32,
    ) -> Result<Program, String> {
        for program in &schedule.programming {
            if time_in_minutes >= program.start_time && time_in_minutes < program.end_time {
                return Ok(program.clone());
            }
        }

        Err("No program is currently playing.".to_string())
    }

    /// Gets the next program to play after a specific time (in minutes since midnight)
    fn get_next_program_at(
        &self,
        schedule: &ChannelSchedule,
        time_in_minutes: i32,
    ) -> Result<Program, String> {
        for program in &schedule.programming {
            if program.start_time >= time_in_minutes {
                return Ok(program.clone());
            }
        }

        Err("No upcoming program.".to_string())
    }

    /// Gets the currently playing program based on the current time and the channel schedule
    pub fn get_now_playing(&self, schedule: &ChannelSchedule) -> Result<Program, String> {
        let current_time = current_time_in_minutes();
        self.get_playing_at(schedule, current_time)
    }

    /// Gets the next program to play after the current program
    pub fn get_next_program(&self, schedule: &ChannelSchedule) -> Result<Program, String> {
        let current_time = current_time_in_minutes();
        self.get_next_program_at(schedule, current_time)
    }
}
