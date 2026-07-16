use jiff::Zoned;

mod core;
mod entities;
mod store;
mod utils;

use core::Core;

fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--build".to_string()) {
        build();
        return;
    }
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
