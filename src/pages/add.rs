use crate::components::ui::Loading;
use crate::tauri_bridge::generate_week;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

#[component]
pub fn Add() -> impl IntoView {
    let (generating, set_generating) = signal(false);

    let on_generate = move |_| {
        set_generating.set(true);
        spawn_local(async move {
            match generate_week().await {
                Ok(_) => {
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().reload();
                    }
                }
                Err(_) => {
                    set_generating.set(false);
                }
            }
        });
    };

    view! {
        <div class="w-full font-sans pb-32">
            <header class="flex items-center justify-between px-6 py-6 sticky top-0 bg-white/90 dark:bg-background-dark/90 backdrop-blur-md z-40">
                <A href="/" attr:class="flex items-center">
                    <span class="material-symbols-outlined">arrow_back_ios</span>
                </A>
                <div class="text-[10px] font-bold tracking-[0.25em] uppercase text-neutral-400 dark:text-zinc-500">Create / V1.0</div>
                <div class="w-6"></div>
            </header>

            <main class="px-6 py-8">
                <section class="mb-12">
                    <h1 class="text-6xl font-extrabold uppercase leading-[0.85] tracking-tighter mb-4">
                        Build <br/> Next
                    </h1>
                    <div class="flex items-center gap-2 text-[10px] font-bold uppercase tracking-widest text-neutral-400 dark:text-zinc-500">
                        <span>"Manual Override Ready"</span>
                        <span class="w-1 h-1 bg-neutral-300 dark:bg-neutral-600 rounded-full"></span>
                        <span>"AI Assisted"</span>
                    </div>
                </section>

                <section class="space-y-6">
                    <div class="p-8 border border-neutral-100 dark:border-neutral-800 bg-zinc-50/50 dark:bg-neutral-900/50">
                        <h2 class="text-xs font-black uppercase tracking-widest mb-2 italic">"Weekly Protocol"</h2>
                        <p class="text-[10px] text-neutral-500 dark:text-neutral-400 leading-relaxed mb-8 uppercase font-bold tracking-tight">
                            "Genera un plan nutricional completo basado en tus preferencias y estado del inventario."
                        </p>

                        <button
                            on:click=on_generate
                            disabled=move || generating.get()
                            class="w-full py-6 bg-primary text-black flex items-center justify-center gap-3 transition-all hover:scale-[1.02] active:scale-[0.98] disabled:opacity-50"
                        >
                            <span class="text-xs font-black uppercase tracking-widest">
                                {move || if generating.get() { "Optimizing System..." } else { "Generate Week Plan" }}
                            </span>
                            <span class="material-symbols-outlined font-bold">"auto_awesome"</span>
                        </button>
                    </div>

                    <div class="grid grid-cols-2 gap-4">
                        <button class="flex flex-col gap-4 p-6 border border-hairline dark:border-neutral-800 hover:bg-zinc-50 dark:hover:bg-neutral-800 transition-all opacity-50 cursor-not-allowed">
                            <span class="material-symbols-outlined text-neutral-400">"restaurant"</span>
                            <span class="text-[10px] font-black uppercase tracking-widest">"Add Single Meal"</span>
                        </button>
                        <button class="flex flex-col gap-4 p-6 border border-hairline dark:border-neutral-800 hover:bg-zinc-50 dark:hover:bg-neutral-800 transition-all opacity-50 cursor-not-allowed">
                            <span class="material-symbols-outlined text-neutral-400">"inventory_2"</span>
                            <span class="text-[10px] font-black uppercase tracking-widest">"Update Inventory"</span>
                        </button>
                    </div>
                </section>
            </main>

            {move || if generating.get() {
                view! {
                    <Portal>
                        <div class="fixed inset-0 bg-white/95 dark:bg-background-dark/95 z-[1000] flex flex-col items-center justify-center">
                            <Loading />
                            <p class="mt-8 text-[10px] font-black uppercase tracking-[0.4em] animate-pulse">"Running AI Logic"</p>
                        </div>
                    </Portal>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
