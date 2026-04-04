use crate::entities::{Channel, Film};

pub fn all_films() -> Vec<Film> {
    vec![
        Film {
            title: "Action Movie 1".to_string(),
            channel_keys: vec!["action".to_string()],
        },
        Film {
            title: "Comedy Movie 1".to_string(),
            channel_keys: vec!["comedy".to_string()],
        },
        Film {
            title: "Crime Movie 1".to_string(),
            channel_keys: vec!["crime".to_string()],
        },
        Film {
            title: "Drama Movie 1".to_string(),
            channel_keys: vec!["drama".to_string()],
        },
        Film {
            title: "Horror Movie 1".to_string(),
            channel_keys: vec!["horror".to_string()],
        },
        Film {
            title: "Romance Movie 1".to_string(),
            channel_keys: vec!["romance".to_string()],
        },
        Film {
            title: "Sci-Fi Movie 1".to_string(),
            channel_keys: vec!["sci-fi".to_string()],
        },
        Film {
            title: "Thriller Movie 1".to_string(),
            channel_keys: vec!["thriller".to_string()],
        },
        Film {
            title: "Westerns Movie 1".to_string(),
            channel_keys: vec!["westerns".to_string()],
        },
        Film {
            title: "Fantasy Movie 1".to_string(),
            channel_keys: vec!["fantasy".to_string()],
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
