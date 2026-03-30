use yew::prelude::*;
mod core;
mod entities;
mod store;

#[component]
fn App() -> Html {
    let core = core::Core::new();

    let date = "2026-03-29";
    let channel_schedules = core.get_channel_schedules(date);

    html! {
        <>
        <h1>{ "Harper TV · Sunday March 29 · 6:37pm" }</h1>
        <div >
            for (channel, _schedule, now_playing) in channel_schedules {
                // let now_playing = core.get_now_playing(&schedule);
                // XXX: Are these clones necessary?
                // TODO: Move styling to CSS file at some point
                <div key={channel.name.clone()} style="display:grid; grid-template-columns: 1fr 2fr;"><div>{channel.name.clone() }</div>
                // XXX: This is really bad, but it works for now. We should probably handle errors properly at some point.
            <div>{ now_playing.map_err(|e| e).unwrap().title}</div>
            </div>

            }
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
