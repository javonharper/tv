use jiff::Zoned;

/// Returns the current time in minutes since midnight
pub fn current_time_in_minutes() -> i32 {
    let now = Zoned::now();
    let hours = now.hour() as i32;
    let minutes = now.minute() as i32;
    hours * 60 + minutes
}
