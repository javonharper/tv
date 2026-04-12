use jiff::Zoned;

/// Returns the current time in minutes since midnight
pub fn current_time_in_minutes() -> i32 {
    let now = Zoned::now();
    let hours = now.hour() as i32;
    let minutes = now.minute() as i32;
    hours * 60 + minutes
}

/// Generates a YouTube search URL for the trailer of a given title
pub fn trailer_url(title: &str) -> String {
    let query = title.replace(" ", "+") + "+trailer";
    format!("https://www.youtube.com/results?search_query={}", query)
}

/// Generates a watch URL for a given title using the template from environment variables
/// Returns None if WATCH_URL_TEMPLATE is not defined in the environment
pub fn watch_url(title: &str) -> Option<String> {
    let template = std::env::var("WATCH_URL_TEMPLATE").ok()?;
    let query = title.replace(" ", "+");
    Some(template.replace("{QUERY}", &query))
}
