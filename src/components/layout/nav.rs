use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

/// Responsive navigation component.
/// - Mobile (< md): top bar (brand + theme + config) + bottom bar (Dashboard, Plan, Add, Calendar, Pantry).
/// - Desktop (>= md): sticky top bar with all navigation options.
#[component]
pub fn Navbar() -> impl IntoView {
    let location = use_location();

    let toggle_dark = move |_| {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(html) = doc.document_element() {
                    let _ = html.class_list().toggle("dark");
                    let is_dark = html.class_list().contains("dark");
                    if let Some(ls) = win.local_storage().ok().flatten() {
                        let _ = ls.set_item("theme", if is_dark { "dark" } else { "light" });
                    }
                }
            }
        }
    };

    view! {
        // ── Desktop top navbar (hidden on mobile) ──────────────────────
        <header class="hidden md:flex items-center justify-between px-6 py-5 sticky top-0 bg-white/80 dark:bg-neutral-950/80 backdrop-blur-md z-40 border-b border-neutral-200/50 dark:border-neutral-800/50">
            // Brand / left section
            <div class="flex items-center gap-4 flex-1">
                <div class="flex flex-col">
                    <div class="text-[10px] font-bold tracking-[0.2em] uppercase">"Status: Optimal"</div>
                    <div class="flex items-center gap-1.5 mt-0.5">
                        <span class="h-[1px] w-3 bg-primary"></span>
                        <span class="text-[7px] font-black text-neutral-400 uppercase tracking-[0.25em]">"nutri.r / Mastery"</span>
                    </div>
                </div>
            </div>

            // Center nav links
            <nav class="flex items-center gap-8 flex-1 justify-center font-medium text-sm">
                <A href="/" attr:class=move || format!("transition-colors {}", if location.pathname.get() == "/" { "text-black dark:text-white font-bold" } else { "text-zinc-400 hover:text-black dark:hover:text-white" })>"Dashboard"</A>
                <A href="/plan" attr:class=move || format!("transition-colors {}", if location.pathname.get().starts_with("/plan") { "text-black dark:text-white font-bold" } else { "text-zinc-400 hover:text-black dark:hover:text-white" })>"Plan"</A>
                <A href="/calendar" attr:class=move || format!("transition-colors {}", if location.pathname.get().starts_with("/calendar") { "text-black dark:text-white font-bold" } else { "text-zinc-400 hover:text-black dark:hover:text-white" })>"Calendar"</A>
                <A href="/pantry" attr:class=move || format!("transition-colors {}", if location.pathname.get().starts_with("/pantry") { "text-black dark:text-white font-bold" } else { "text-zinc-400 hover:text-black dark:hover:text-white" })>"Pantry"</A>
            </nav>

            // Right actions
            <div class="flex items-center gap-3 flex-1 justify-end">
                <A href="/add" attr:class="flex bg-primary text-black px-4 py-1.5 rounded-full text-xs font-bold hover:scale-105 transition-all mr-2 shadow-sm">
                    "Add Meal"
                </A>
                <button
                    on:click=toggle_dark
                    class="w-10 h-10 flex items-center justify-center rounded-full hover:bg-zinc-100 dark:hover:bg-neutral-800 transition-colors"
                >
                    <span class="material-symbols-outlined text-xl">"contrast"</span>
                </button>
                <A href="/config" attr:class="w-8 h-8 bg-neutral-950 dark:bg-white rounded-full flex items-center justify-center overflow-hidden active:scale-95 transition-transform">
                    <span class="material-symbols-outlined text-white dark:text-black text-sm">"person"</span>
                </A>
            </div>
        </header>

        // ── Mobile top bar (hidden on desktop) ─────────────────────────
        <header class="md:hidden pt-safe flex items-center justify-between px-5 py-3 sticky top-0 bg-white/90 dark:bg-neutral-950/90 backdrop-blur-xl z-40 border-b border-neutral-200/30 dark:border-neutral-800/30">
            <div class="flex flex-col">
                <div class="text-xs font-bold tracking-[0.15em] uppercase">"nutri.r"</div>
                <div class="flex items-center gap-1.5 mt-0.5">
                    <span class="h-[1px] w-3 bg-primary"></span>
                    <span class="text-[7px] font-black text-neutral-400 uppercase tracking-[0.25em]">"Mastery"</span>
                </div>
            </div>
            <div class="flex items-center gap-2">
                <button
                    on:click=toggle_dark
                    class="w-9 h-9 flex items-center justify-center rounded-full hover:bg-zinc-100 dark:hover:bg-neutral-800 transition-colors"
                >
                    <span class="material-symbols-outlined text-lg">"contrast"</span>
                </button>
                <A href="/config" attr:class=move || format!("w-8 h-8 rounded-full flex items-center justify-center overflow-hidden active:scale-95 transition-all {}",
                    if location.pathname.get() == "/config" { "bg-primary" } else { "bg-neutral-950 dark:bg-white" })>
                    <span class=move || format!("material-symbols-outlined text-sm {}",
                        if location.pathname.get() == "/config" { "text-black" } else { "text-white dark:text-black" })>"person"</span>
                </A>
            </div>
        </header>

        // ── Mobile bottom navbar (hidden on desktop) ───────────────────
        <nav class="md:hidden fixed bottom-0 left-0 right-0 bg-white/90 dark:bg-neutral-950/90 backdrop-blur-xl border-t border-neutral-200/30 dark:border-neutral-800/30 px-6 pb-10 pt-4 pb-safe z-40">
            <div class="flex items-center justify-between max-w-md mx-auto relative">
                <BottomNavLink
                    href="/"
                    icon="grid_view"
                    active=Memo::new(move |_| location.pathname.get() == "/")
                />
                <BottomNavLink
                    href="/plan"
                    icon="analytics"
                    active=Memo::new(move |_| location.pathname.get().starts_with("/plan"))
                />

                <div class="flex items-center">
                    <A
                        href="/add"
                        attr:class="bg-primary text-black w-10 h-10 flex items-center justify-center rounded-full shadow-sm hover:scale-105 active:scale-95 transition-all"
                    >
                        <span class="material-symbols-outlined text-2xl font-bold">"add"</span>
                    </A>
                </div>

                <BottomNavLink
                    href="/calendar"
                    icon="calendar_today"
                    active=Memo::new(move |_| location.pathname.get().starts_with("/calendar"))
                />
                <BottomNavLink
                    href="/pantry"
                    icon="kitchen"
                    active=Memo::new(move |_| location.pathname.get().starts_with("/pantry"))
                />
            </div>
        </nav>
    }
}

#[component]
fn BottomNavLink(href: &'static str, icon: &'static str, active: Memo<bool>) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class=move || format!("p-2 transition-colors relative {}",
                if active.get() { "text-black dark:text-white" } else { "text-zinc-400 hover:text-black dark:hover:text-white" })
        >
            <span class="material-symbols-outlined text-2xl font-light">
                {icon}
            </span>
            {move || if active.get() {
                view! { <div class="absolute -bottom-1 left-1/2 -translate-x-1/2 w-1 h-1 bg-primary rounded-full"></div> }.into_any()
            } else {
                ().into_any()
            }}
        </A>
    }
}
