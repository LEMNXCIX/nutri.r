use crate::components::layout::{BottomNav, Navbar};
use crate::components::ui::Toast;
use crate::pages::achievements::Achievements;
use crate::pages::calendar::Calendar;
use crate::pages::config::Config;
use crate::pages::dashboard::Dashboard;
use crate::pages::favorites::Favorites;
use crate::pages::history::History;
use crate::pages::home::Home;
use crate::pages::ingredients::Ingredients;
use crate::pages::pantry::Pantry;
use crate::pages::plan_detail::PlanDetail;
use crate::pages::shopping_list::ShoppingList;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn App() -> impl IntoView {
    let (toast_msg, set_toast_msg) = signal(String::new());
    let (is_error, set_is_error) = signal(false);

    Effect::new(move |_| {
        let cb = Closure::wrap(Box::new(move |ev: web_sys::CustomEvent| {
            let detail = ev.detail();
            let msg = js_sys::Reflect::get(&detail, &JsValue::from_str("message"))
                .unwrap_or(JsValue::NULL)
                .as_string()
                .unwrap_or_default();

            let err = js_sys::Reflect::get(&detail, &JsValue::from_str("is_error"))
                .unwrap_or(JsValue::NULL)
                .as_bool()
                .unwrap_or(false);

            set_toast_msg.set(msg);
            set_is_error.set(err);

            // Use a timeout to clear the toast
            let set_msg = set_toast_msg.clone();
            if let Some(win) = web_sys::window() {
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                    Closure::once_into_js(move || {
                        set_msg.set(String::new());
                    })
                    .as_ref()
                    .unchecked_ref(),
                    3000,
                );
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback(
                "toast-notification",
                cb.as_ref().unchecked_ref(),
            );
            cb.forget();
        }
    });

    view! {
        <Router>
            <div class="min-h-screen bg-white text-black">
                <Navbar />
                <main class="container mx-auto px-4 py-8 pb-32 md:pb-8">
                    <Routes fallback=|| "Not Found">
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/achievements") view=Achievements />
                        <Route path=path!("/dashboard") view=Dashboard />
                        <Route path=path!("/favorites") view=Favorites />
                        <Route path=path!("/history") view=History />
                        <Route path=path!("/plan/:id") view=PlanDetail />
                        <Route path=path!("/shopping/:id") view=ShoppingList />
                        <Route path=path!("/calendar") view=Calendar />
                        <Route path=path!("/config") view=Config />
                        <Route path=path!("/ingredients") view=Ingredients />
                        <Route path=path!("/pantry") view=Pantry />
                    </Routes>
                </main>

                <BottomNav />
                {move || if !toast_msg.get().is_empty() {
                    view! {
                        <Toast
                            message=Signal::derive(move || toast_msg.get())
                            on_close=Callback::new(move |_| set_toast_msg.set(String::new()))
                            is_error=is_error.get()
                        />
                    }.into_any()
                } else {
                    ().into_any()
                }}
            </div>
        </Router>
    }
}
