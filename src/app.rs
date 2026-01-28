use crate::components::navbar::Navbar;
use crate::pages::config::Config;
use crate::pages::home::Home;
use crate::pages::plan::Plan;
use crate::pages::plan_detail::PlanDetail;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar />
            <main class="container mx-auto p-4">
                <Routes fallback=|| "Not Found">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/plan/:id") view=PlanDetail />
                    <Route path=path!("/config") view=Config />
                    <Route path=path!("/plans") view=Plan />
                </Routes>
            </main>
        </Router>
    }
}
