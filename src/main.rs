use yew::prelude::*;
mod core;
mod entities;

#[component]
fn App() -> Html {
    let core = core::Core::new();

    let date = "2026-03-29";
    let channel_schedules = core.get_channel_schedules(date);

    html! {
        <>
        <h1>{ "Harper TV · Sunday March 29 · 6:37pm" }</h1>
        // <div>{ "wassup" }</div>
        <div>
            for channel in &channel_schedules {
                <p key={channel.clone()}>{ channel }</p>
            }
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
