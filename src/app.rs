use crate::components::layout::Navbar;
use crate::components::ui::Toast;
use crate::pages::achievements::Achievements;
use crate::pages::add::Add;
use crate::pages::calendar::Calendar;
use crate::pages::config::Config;
use crate::pages::daily_view::DailyView;
use crate::pages::dashboard::Dashboard;
use crate::pages::favorites::Favorites;
use crate::pages::history::History;
use crate::pages::home::Home;
use crate::pages::ingredients::Ingredients;
use crate::pages::pantry::Pantry;
use crate::pages::plan::Plan;
use crate::pages::plan_detail::PlanDetail;
use crate::pages::recipe_detail::RecipeDetail;
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
        crate::theme::apply_theme(&crate::theme::initial_theme());
    });

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

    Effect::new(move |_| {
        let cb = Closure::wrap(Box::new(move |_ev: web_sys::Event| {
            leptos::task::spawn_local(async {
                crate::tauri_bridge::log_trace(
                    "NET: Connection restored, triggering sync".to_string(),
                );
                crate::tauri_bridge::auto_pull().await;
                crate::tauri_bridge::auto_push().await;
            });
        }) as Box<dyn FnMut(_)>);

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("online", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    Effect::new(move |_| {
        leptos::task::spawn_local(async {
            // Do an immediate health check first so IS_API_ONLINE is accurate
            // before any page data fetch happens, then start the periodic loop.
            crate::tauri_bridge::check_health().await;
            crate::tauri_bridge::start_health_check_loop().await;
            crate::tauri_bridge::auto_pull().await;
        });
    });

    view! {
        <Router>
            <div class="min-h-screen bg-white text-neutral-950 dark:bg-background-dark dark:text-white selection:bg-accent selection:text-neutral-950">
                <Navbar />
                <main class="w-full pb-32 md:pb-0">
                    <Routes fallback=|| "Not Found">
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/add") view=Add />
                        <Route path=path!("/achievements") view=Achievements />
                        <Route path=path!("/dashboard") view=Dashboard />
                        <Route path=path!("/favorites") view=Favorites />
                        <Route path=path!("/history") view=History />
                        <Route path=path!("/plan") view=Plan />
                        <Route path=path!("/plan/:id") view=PlanDetail />
                        <Route path=path!("/plan/:id/recipe/:recipe_id") view=RecipeDetail />
                        <Route path=path!("/shopping/:id") view=ShoppingList />
                        <Route path=path!("/calendar") view=Calendar />
                        <Route path=path!("/calendar/:date") view=DailyView />
                        <Route path=path!("/config") view=Config />
                        <Route path=path!("/ingredients") view=Ingredients />
                        <Route path=path!("/pantry") view=Pantry />
                    </Routes>
                </main>


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
