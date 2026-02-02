use crate::components::layout::{BottomNav, Navbar};
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
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
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
                        <Route path=path!("/plan/:id") view=PlanDetail />
                        <Route path=path!("/shopping/:id") view=ShoppingList />
                        <Route path=path!("/calendar") view=Calendar />
                        <Route path=path!("/config") view=Config />
                        <Route path=path!("/ingredients") view=Ingredients />
                        <Route path=path!("/pantry") view=Pantry />
                    </Routes>
                </main>
                <BottomNav />
            </div>
        </Router>
    }
}
