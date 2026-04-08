use crate::tauri_bridge::{get_config, get_ui_preferences, AppConfig, UIPreferences, SyncStatus, get_sync_status, check_health};
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct AppContext {
    pub config: ReadSignal<AppConfig>,
    pub set_config: WriteSignal<AppConfig>,
    pub preferences: ReadSignal<UIPreferences>,
    pub set_preferences: WriteSignal<UIPreferences>,
    pub sync_status: ReadSignal<SyncStatus>,
    pub set_sync_status: WriteSignal<SyncStatus>,
    pub is_online: ReadSignal<bool>,
    pub set_is_online: WriteSignal<bool>,
    pub data_changed_signal: ReadSignal<u32>,
    pub notify_data_changed: WriteSignal<u32>,
}

pub fn provide_app_context() {
    let (config, set_config) = signal(AppConfig::default());
    let (preferences, set_preferences) = signal(UIPreferences::default());
    let (sync_status, set_sync_status) = signal(SyncStatus::Idle);
    let (is_online, set_is_online) = signal(true);
    let (data_changed_signal, notify_data_changed) = signal(0_u32);

    let context = AppContext {
        config,
        set_config,
        preferences,
        set_preferences,
        sync_status,
        set_sync_status,
        is_online,
        set_is_online,
        data_changed_signal,
        notify_data_changed,
    };

    // Listen for data-changed events from Tauri
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::Closure;
        use wasm_bindgen::JsCast;
        let window = web_sys::window().unwrap();
        let cb = Closure::wrap(Box::new(move |ev: web_sys::CustomEvent| {
            crate::tauri_bridge::log_trace("DATA_CHANGED: Event received from Tauri".to_string());
            notify_data_changed.update(|n| *n += 1);
        }) as Box<dyn FnMut(web_sys::CustomEvent)>);

        let _ = window.add_event_listener_with_callback("data-changed", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // Initialize state
    spawn_local(async move {
        if let Ok(c) = get_config().await {
            set_config.set(c);
        }
        if let Ok(p) = get_ui_preferences().await {
            set_preferences.set(p);
        }
        if let Ok(s) = get_sync_status().await {
            set_sync_status.set(s);
        }
    });

    // Health Check Loop integrated into Context
    spawn_local(async move {
        loop {
            let online = check_health().await;
            set_is_online.set(online);
            gloo_timers::future::sleep(Duration::from_secs(30)).await;
        }
    });

    provide_context(context);
}

pub fn use_app_context() -> AppContext {
    use_context::<AppContext>().expect("AppContext must be provided in the root component")
}
