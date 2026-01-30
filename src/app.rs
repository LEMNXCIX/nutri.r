use crate::components::layout::Navbar;
use crate::pages::achievements::Achievements;
use crate::pages::calendar::Calendar;
use crate::pages::config::Config;
use crate::pages::dashboard::Dashboard;
use crate::pages::favorites::Favorites;
use crate::pages::home::Home;
use crate::pages::ingredients::Ingredients;
use crate::pages::pantry::Pantry;
use crate::pages::plan_detail::PlanDetail;
use crate::pages::shopping_list::ShoppingList;
use crate::tauri_bridge::get_ui_preferences;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    let (_theme, set_theme) = signal("light".to_string());

    spawn_local(async move {
        match get_ui_preferences().await {
            Ok(prefs) => {
                set_theme.set(prefs.theme.clone());
                if prefs.theme == "dark" {
                    let document = web_sys::window().unwrap().document().unwrap();
                    let html = document.document_element().unwrap();
                    let _ = html.class_list().add_1("dark");
                }
            }
            Err(e) => log::error!("Failed to load preferences: {}", e),
        }
    });

    view! {
        <Router>
            <div class="min-h-screen bg-gray-50 dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors">
                <Navbar />
                <main class="container mx-auto p-4 transition-all pb-12">
                    <Routes fallback=|| "Not Found">
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/achievements") view=Achievements />
                        <Route path=path!("/dashboard") view=Dashboard />
                        <Route path=path!("/favorites") view=Favorites />
                        <Route path=path!("/plan/:id") view=PlanDetail />
                        <Route path=path!("/shopping/:id") view=ShoppingList />
                        <Route path=path!("/calendar") view=Calendar />
                        <Route path=path!("/config") view=Config />
                        <Route path=path!("/ingredients") view=Ingredients />
                        <Route path=path!("/pantry") view=Pantry />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
