use axum::{Router, routing::get};
use jiff::{Unit, Zoned};
use maud::{Markup, html};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

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

    let channels = vec!["Channel 1", "Channel 2", "Channel 3"];

    let markup = html! {
        title { (name) " · " (date_short)}
        link rel="stylesheet" href="/styles.css" {}

        p { (name) " · " (date) " · " (time) }
        div.grid {
            @for channel in channels {
                div { (channel) }
                div { "Program 1" }
            }
         }
    };

    markup
}
