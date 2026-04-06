use axum::{Router, routing::get};
use jiff::{Unit, Zoned};
use maud::{Markup, html};
use tower_http::services::ServeDir;

mod core;
mod entities;
mod store;
mod utils;

use core::Core;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

async fn handler() -> Markup {
    let name = "Harper's Television";

    let now = Zoned::now().round(Unit::Second).unwrap();
    let date = now.strftime("%A %B %e");
    let date_short = now.strftime("%b %e");
    let time = now.strftime("%-I:%M%P");

    let core = Core::new();
    let today_seed = now.strftime("%Y-%m-%d").to_string();
    let channel_schedules = core.get_channel_schedules(today_seed);

    let markup = html! {
        title { (name) " · " (date_short)}
        link rel="stylesheet" type="text/css" href="./reset.css" {}
        link rel="stylesheet" type="text/css" href="./styles.css" {}

        p { (name) " · " (date) " · " (time) }
        div.grid {
            div.row.header {
                div {  }
                div { "Now Playing" }
                // div { "Next Up" }
            }
            @for channel_schedule in channel_schedules {
                div.row {
                    div { (channel_schedule.channel.name) }
                    div {
                        @if let Some(now_playing) = channel_schedule.now_playing {
                        (now_playing.title)
                        } @else {
                            "No program currently playing."
                        }
                    }
                    // div {
                    //     @if let Some(next_up) = channel_schedule.next_up {
                    //         "Next: " (next_up.title)
                    //     } @else {
                    //         "No upcoming program."
                    //     }
                    // }
                }
            }
         }
    };

    markup
}
