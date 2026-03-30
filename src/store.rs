use crate::entities::Film;

pub fn all_films() -> Vec<Film> {
    vec![
        Film {
            title: "In Too Deep".to_string(),
            year: "1999".to_string(),
            runtime: "2h 0m".to_string(),
        },
        Film {
            title: "New Jack City".to_string(),
            year: "1991".to_string(),
            runtime: "2h 0m".to_string(),
        },
        Film {
            title: "Clockers".to_string(),
            year: "1995".to_string(),
            runtime: "2h 0m".to_string(),
        },
    ]
}
