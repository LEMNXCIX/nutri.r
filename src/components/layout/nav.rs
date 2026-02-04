use crate::components::ui::SyncStatus;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-[100] bg-white/80 backdrop-blur-md border-b border-gray-200">
            <div class="max-w-7xl mx-auto flex items-center justify-between px-6 py-4">
                // Branding
                <A href="/" attr:class="flex items-center gap-3 group">
                    <div class="w-10 h-10 bg-black rounded-xl flex items-center justify-center transition-transform group-hover:scale-105">
                        <span class="text-white font-bold text-xl">"n"</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-lg font-semibold tracking-tight">"nutri.r"</span>
                        <span class="text-[10px] font-medium text-gray-500 uppercase tracking-wide">"Personal"</span>
                    </div>
                </A>

                // Desktop Navigation
                <nav class="hidden md:block">
                    <ul class="flex items-center gap-1">
                        <li><NavLink href="/" label="Inicio" /></li>
                        <li><NavLink href="/favorites" label="Favoritos" /></li>
                        <li><NavLink href="/dashboard" label="Dashboard" /></li>
                        <li><NavLink href="/calendar" label="Calendario" /></li>
                        <li><NavLink href="/ingredients" label="Ingredientes" /></li>
                        <li><NavLink href="/pantry" label="Despensa" /></li>
                    </ul>
                </nav>

                // System Actions
                <div class="flex items-center gap-2">
                    <SyncStatus />
                    <A href="/achievements" attr:class="p-2.5 rounded-lg hover:bg-gray-100 text-gray-600 hover:text-black transition-all" attr:title="Logros">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
                        </svg>
                    </A>
                    <A href="/config" attr:class="p-2.5 rounded-lg hover:bg-gray-100 text-gray-600 hover:text-black transition-all" attr:title="Configuración">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    </A>
                </div>
            </div>
        </header>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="px-3 py-2 rounded-lg text-sm font-medium text-gray-600 hover:text-black hover:bg-gray-100 transition-all"
        >
            {label}
        </A>
    }
}
