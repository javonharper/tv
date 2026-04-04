use crate::entities::{Channel, Film};

pub fn all_films() -> Vec<Film> {
    vec![
        Film {
            title: "In Too Deep".to_string(),
        },
        Film {
            title: "New Jack City".to_string(),
        },
        Film {
            title: "Clockers".to_string(),
        },
    ]
}

pub fn all_channels() -> Vec<Channel> {
    vec![
        Channel::new("Action"),
        Channel::new("Comedy"), // Belly Laugh
        Channel::new("Crime"),
        Channel::new("Drama"),
        Channel::new("Horror"),
        Channel::new("Romance"), // That Feeling
        Channel::new("Sci-Fi"),
        Channel::new("Thriller"),
        Channel::new("Westerns"),
        Channel::new("Fantasy"),
    ]
}
