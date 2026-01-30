use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-[100] px-4 py-4 pointer-events-none">
            <div class="max-w-7xl mx-auto flex items-center justify-between pointer-events-auto glass rounded-[2rem] px-6 py-3 ring-1 ring-white/5 shadow-2xl">
                // Logo Section
                <A href="/" attr:class="flex items-center gap-3 group">
                    <div class="w-10 h-10 bg-gradient-to-br from-green-400 to-green-600 rounded-2xl flex items-center justify-center shadow-lg shadow-green-500/20 group-hover:scale-110 transition-transform">
                        <span class="text-white font-black text-2xl tracking-tighter">"n"</span>
                    </div>
                    <div class="flex flex-col -space-y-1">
                        <span class="text-lg font-black text-white tracking-tighter uppercase">"nutri.r"</span>
                        <span class="text-[8px] font-black text-green-500 tracking-[0.3em] uppercase opacity-80">"Premium"</span>
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
                    <A href="/achievements" attr:class="p-2.5 rounded-xl hover:bg-gray-800/50 text-gray-400 hover:text-yellow-400 transition-all" attr:title="Logros">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
                        </svg>
                    </A>
                    <A href="/config" attr:class="p-2.5 rounded-xl hover:bg-gray-800/50 text-gray-400 hover:text-green-400 transition-all" attr:title="Configuración">
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
            attr:class="px-4 py-2 rounded-xl text-xs font-black text-gray-400 hover:text-white transition-all uppercase tracking-widest hover:bg-white/5 active:scale-95"
        >
            {label}
        </A>
    }
}
