use crate::{
    entities::{Channel, ChannelSchedule, ChannelScheduleResponse, Program},
    store::{all_channels, all_films},
};

pub struct Core {
    channels: Vec<Channel>,
}

impl Core {
    pub fn new() -> Self {
        let channels = all_channels();

        Self { channels }
    }

    /// Gets the following information for each channel:
    /// - Channel object
    /// - Channel schedule for the given date
    /// - Currently playing program (if any)
    pub fn get_channel_schedules(
        &self,
        date: &str,
        // XXX: String error :thumbsdown:
    ) -> Vec<ChannelScheduleResponse> {
        self.channels
            .iter()
            .map(|channel| {
                let schedule = self.build_channel_schedule(date, channel);
                let now_playing = self.get_now_playing(&schedule);
                ChannelScheduleResponse {
                    channel: channel.clone(),
                    programming: schedule.programming,
                    now_playing: now_playing.ok(),
                }
            })
            .collect()
    }

    /// Builds a channel schedule for a given channel.
    pub fn build_channel_schedule(&self, _date: &str, _channel: &Channel) -> ChannelSchedule {
        // let films = all_films();
        // let mut schedule = ChannelSchedule::new();

        // // From midnight until one minute before the end of the day, we loop through the films and add add them to the schedule as programs. If we reach the end of the film list, we start over from the beginning until we've filled the schedule for the entire day.
        // let start_of_day = "00:00".to_string();
        // let end_of_day = "23:59".to_string();
        // let mut current_time = start_of_day.clone();

        // while current_time < end_of_day {
        //     let mut rng = rand::rng();

        //     let film = match films.choose(&mut rng) {
        //         Some(film) => film,
        //         None => panic!("No films available to choose from."),
        //     };

        //     let program = Program {
        //         title: film.title.clone(),
        //         start_time: current_time.clone(),
        //         end_time: "23:59".to_string(),
        //     };

        //     schedule.programming.push(program);

        //     // Increment the current time by 2 hours (the assumed runtime of each film)
        //     let current_time_hours: u32 = current_time[0..2].parse().unwrap();
        //     let current_time_minutes: u32 = current_time[3..5].parse().unwrap();
        //     let new_time_hours = (current_time_hours + 2) % 24;
        //     let new_time_minutes = current_time_minutes;
        //     current_time = format!("{:02}:{:02}", new_time_hours, new_time_minutes);
        // }

        // schedule
        ChannelSchedule::new()
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
