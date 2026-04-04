use axum::{Router, routing::get};
use jiff::{Unit, Zoned};
use maud::{Markup, html};
use tower_http::services::ServeDir;

mod core;
mod entities;
mod store;

use core::Core;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
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
    let channel_schedules =
        core.get_channel_schedules(now.strftime("%Y-%m-%d").to_string().as_str());

    let markup = html! {
        title { (name) " · " (date_short)}
        link rel="stylesheet" type="text/css" href="./styles.css" {}

        p { (name) " · " (date) " · " (time) }
        div.grid {
            @for channel_schedule in channel_schedules {
                div.row {
                    div { (channel_schedule.channel.name) }
                    div { "Program 1" }
                }
            }
         }
    };

    markup
}
