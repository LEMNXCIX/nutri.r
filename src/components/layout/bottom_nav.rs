use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn BottomNav() -> impl IntoView {
    let location = use_location();

    view! {
        <nav class="md:hidden fixed bottom-6 left-6 right-6 z-[100] bg-white rounded-[2rem] shadow-soft-lg border border-gray-100">
            <ul class="flex items-center justify-between px-6 py-4">
                <li><BottomNavLink href="/" icon="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" label="Home" active=Memo::new(move |_| location.pathname.get() == "/") /></li>
                <li><BottomNavLink href="/favorites" icon="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" label="Heart" active=Memo::new(move |_| location.pathname.get() == "/favorites") /></li>
                <li><BottomNavLink href="/dashboard" icon="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" label="Progress" active=Memo::new(move |_| location.pathname.get() == "/dashboard") /></li>
                <li><BottomNavLink href="/calendar" icon="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" label="Plan" active=Memo::new(move |_| location.pathname.get() == "/calendar") /></li>
                <li><BottomNavLink href="/ingredients" icon="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" label="Food" active=Memo::new(move |_| location.pathname.get() == "/ingredients") /></li>
                <li><BottomNavLink href="/pantry" icon="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" label="Pantry" active=Memo::new(move |_| location.pathname.get() == "/pantry") /></li>
            </ul>
        </nav>
    }
}

#[component]
fn BottomNavLink(
    href: &'static str,
    icon: &'static str,
    label: &'static str,
    active: Memo<bool>,
) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class=move || format!("flex flex-col items-center gap-1 transition-all {}",
                if active.get() { "text-black scale-105" } else { "text-gray-400 hover:text-gray-600" })
        >
            <svg class="w-6 h-6" fill=move || if active.get() { "currentColor" } else { "none" } stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d=icon />
            </svg>
            <span class="text-[10px] font-medium">{label}</span>
        </A>
    }
}
