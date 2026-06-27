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
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--build".to_string()) {
        build();
        return;
    }

    let app = Router::new()
        .route("/", get(handler))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn build() {
    let now = Zoned::now();
    let today_seed = now.strftime("%Y-%m-%d").to_string();

    let core = Core::new();
    let schedules = core.get_full_schedules(today_seed);

    let json_channels: Vec<String> = schedules
        .iter()
        .map(|(channel, schedule)| {
            let programs: Vec<String> = schedule
                .programming
                .iter()
                .map(|p| {
                    let url = utils::watch_url(&p.title)
                        .map(|u| format!("\"{}\"", json_escape(&u)))
                        .unwrap_or_else(|| "null".to_string());
                    format!(
                        "{{\"title\":\"{}\",\"start\":{},\"end\":{},\"url\":{}}}",
                        json_escape(&p.title),
                        p.start_time,
                        p.end_time,
                        url
                    )
                })
                .collect();
            format!(
                "{{\"channel\":\"{}\",\"key\":\"{}\",\"programs\":[{}]}}",
                json_escape(&channel.name),
                json_escape(&channel.key),
                programs.join(",")
            )
        })
        .collect();

    let schedule_json = format!("[{}]", json_channels.join(","));

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <title>Harper's Television</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link rel="stylesheet" type="text/css" href="./reset.css">
  <link rel="stylesheet" type="text/css" href="./styles.css">
</head>
<body>
  <p class="heading">
    Harper's Television &middot;
    <span id="client-date"></span>
    &middot;
    <span id="client-time"></span>
  </p>
  <div class="grid" id="grid"></div>
  <script>const SCHEDULE = {};</script>
  <script src="./script.js"></script>
</body>
</html>"#,
        schedule_json
    );

    std::fs::create_dir_all("dist").unwrap();
    std::fs::write("dist/index.html", html).unwrap();
    println!("Built dist/index.html");
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
        title { (name) " · " (date_short)}        meta name="viewport" content="width=device-width, initial-scale=1.0";        link rel="stylesheet" type="text/css" href="./reset.css" {}
        link rel="stylesheet" type="text/css" href="./styles.css" {}

        p.heading {
            (name) " · "
            span #client-date { (date) }
            " · "
            span #client-time { (time) }
        }
        script src="./script.js" {}
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
                            @if let Some(url) = utils::watch_url(&now_playing.title) {
                                a href=(url) target="_blank" {
                                    (now_playing.title)
                                }
                            } @else {
                                span.movie-title data-title=(now_playing.title) {
                                    (now_playing.title)
                                }
                            }
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
