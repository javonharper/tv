use yew::prelude::*;
mod core;
mod entities;
mod store;

#[component]
fn App() -> Html {
    // let _core = core::Core::new();

    let heading = "Harper TV · Sunday March 29 · 6:37pm";

    // info!("Rending app for date: {}", heading);

    // let _date = "2026-03-29";
    // let channel_schedules = core.get_channel_schedules(date);

    html! {

            <h1>{ heading }</h1>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
