use crate::entities::{Channel, Film};
use std::fs::File;

pub fn all_films() -> Vec<Film> {
    let file = File::open("films.csv").expect("Could not open films.csv");
    let mut reader = csv::Reader::from_reader(file);

    let mut films = Vec::new();
    for result in reader.records() {
        let record = result.expect("Error parsing CSV record");
        let title = record.get(0).expect("Missing title column").to_string();
        let collections_str = record.get(1).expect("Missing collections column");
        let enabled_str = record.get(2).expect("Missing enabled column");
        let enabled = enabled_str.trim().eq_ignore_ascii_case("true");

        let collections: Vec<String> = collections_str
            .split(';')
            .map(|s| s.trim().to_string())
            .collect();

        if enabled {
            films.push(Film { title, collections });
        } else {
            println!("Skipping disabled film: {}", title);
        }
    }

    films
}

pub fn all_channels() -> Vec<Channel> {
    let file = File::open("channels.csv").expect("Could not open channels.csv");
    let mut reader = csv::Reader::from_reader(file);

    let mut channels = Vec::new();
    for result in reader.records() {
        let record = result.expect("Error parsing CSV record");
        let key = record.get(0).expect("Missing key column").to_string();
        let name = record.get(1).expect("Missing name column").to_string();
        let enabled_str = record.get(2).expect("Missing enabled column");
        let enabled = enabled_str.trim().eq_ignore_ascii_case("true");

        if enabled {
            channels.push(Channel { key, name });
        } else {
            println!("Skipping disabled channel: {}", name);
        }
    }

    channels
}
