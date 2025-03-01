use leptos::task::spawn_local;
use leptos::*;
use leptos::{ev::SubmitEvent, prelude::*};
use scoreboard_ui::invoke;
use serde::{Deserialize, Serialize};
use thaw::{Button, ButtonType};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg(new_msg);
        });
    };

    view! {
        <main>
            <h1>"Welcome to Tauri + Leptos"</h1>
            <form on:submit=greet>
                <input id="greet-input" placeholder="Enter a name..." on:input=update_name />
                <Button button_type=ButtonType::Submit>"Greet"</Button>
            </form>
            <p class="text-red-500">{move || greet_msg()}</p>
        </main>
    }
}
