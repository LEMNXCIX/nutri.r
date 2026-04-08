mod api;
mod app;
mod components;
mod pages;
mod plan_display;
mod state;
mod tauri;
mod tauri_bridge;
mod theme;
mod types;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
