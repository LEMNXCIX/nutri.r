use crate::tauri_bridge::{get_sync_status, SyncStatus as Status};
use leptos::prelude::*;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Deserialize)]
pub struct SyncStatusPayload {
    pub status: Status,
}

#[component]
pub fn SyncStatus() -> impl IntoView {
    let (status, set_status) = signal(Status::Idle);

    // Initial load
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(res) = get_sync_status().await {
                set_status.set(res);
            }
        });
    });

    // Event listener for Tauri events
    Effect::new(move |_| {
        let cb = Closure::wrap(Box::new(move |ev: web_sys::CustomEvent| {
            let detail = ev.detail();
            if let Ok(payload_js) = js_sys::Reflect::get(&detail, &JsValue::from_str("status")) {
                if let Ok(new_status) = serde_wasm_bindgen::from_value::<Status>(payload_js) {
                    set_status.set(new_status);
                }
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback(
                "sync-status-changed",
                cb.as_ref().unchecked_ref(),
            );
            cb.forget();
        }
    });

    view! {
        <div class="flex items-center gap-2 px-2.5 py-1.5 rounded-full bg-gray-50 dark:bg-neutral-900 border border-gray-100 dark:border-neutral-800 shadow-sm transition-all duration-300 hover:bg-white dark:hover:bg-neutral-800 hover:shadow-md">
            {move || match status.get() {
                Status::Idle => view! {
                    <div class="w-2 h-2 rounded-full bg-gray-300 dark:bg-neutral-600"></div>
                    <span class="text-[10px] font-bold text-gray-400 dark:text-neutral-500 uppercase tracking-widest leading-none">"Ready"</span>
                }.into_any(),
                Status::Syncing => view! {
                    <div class="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></div>
                    <span class="text-[10px] font-bold text-blue-500 uppercase tracking-widest leading-none">"Syncing"</span>
                }.into_any(),
                Status::Success => view! {
                    <div class="w-2 h-2 rounded-full bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.4)]"></div>
                    <span class="text-[10px] font-bold text-green-600 uppercase tracking-widest leading-none">"Synced"</span>
                }.into_any(),
                Status::Error(_) => view! {
                    <div class="w-2 h-2 rounded-full bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.4)]"></div>
                    <span class="text-[10px] font-bold text-red-500 uppercase tracking-widest leading-none">"Offline"</span>
                }.into_any(),
            }}
        </div>
    }
}
