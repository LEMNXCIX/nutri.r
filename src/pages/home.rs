use crate::components::features::PlanList;
use crate::components::ui::Button;
use crate::tauri_bridge::{generate_week, search_plans, SearchFilters};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Home() -> impl IntoView {
    // Estado
    let (loading, set_loading) = signal(false);
    let (filters, set_filters) = signal(SearchFilters::default());

    let plans_resource = LocalResource::new(move || {
        let f = filters.get();
        async move { search_plans(f).await.unwrap_or_default() }
    });

    let plans = Signal::derive(move || plans_resource.get().unwrap_or_default());

    let on_generate = move |_| {
        set_loading.set(true);
        spawn_local(async move {
            match generate_week().await {
                Ok(_) => {
                    leptos::logging::log!("Plan generado!");
                    plans_resource.refetch();
                }
                Err(e) => leptos::logging::error!("Error: {}", e),
            }
            set_loading.set(false);
        });
    };

    let on_click_handler = Callback::new(on_generate);
    let disabled_signal = Signal::derive(move || loading.get());

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 animate-in fade-in duration-700">
            // Hero Section
            <div class="relative overflow-hidden rounded-[2.5rem] bg-gray-900 border border-gray-800 shadow-2xl mb-12 group">
                <div class="absolute inset-0 bg-gradient-to-br from-green-500/10 via-transparent to-purple-500/10 opacity-50"></div>
                <div class="absolute -top-24 -right-24 w-96 h-96 bg-green-500/10 blur-[100px] rounded-full"></div>
                <div class="absolute -bottom-24 -left-24 w-96 h-96 bg-purple-500/10 blur-[100px] rounded-full"></div>

                <div class="relative px-8 py-16 md:px-16 md:py-20 flex flex-col md:flex-row items-center gap-12">
                    <div class="flex-1 text-center md:text-left">
                        <span class="inline-block px-4 py-1.5 rounded-full bg-green-500/10 text-green-400 text-xs font-black uppercase tracking-[0.2em] mb-6">
                            "AI-Powered Nutrition"
                        </span>
                        <h1 class="text-4xl md:text-6xl font-black text-white tracking-tighter mb-6 leading-[1.1]">
                            "Tu salud, "
                            <span class="premium-gradient-text italic">"optimizada"</span>
                            <br/>
                            "por IA."
                        </h1>
                        <p class="text-gray-400 text-lg mb-8 max-w-xl mx-auto md:mx-0 leading-relaxed font-medium">
                            "Genera planes nutricionales personalizados en segundos. Inteligencia artificial diseñada para tu bienestar diario."
                        </p>
                        <div class="flex flex-wrap justify-center md:justify-start gap-4">
                            <Button
                                on_click=on_click_handler
                                disabled=disabled_signal
                                class="px-8 py-4 rounded-2xl bg-green-500 hover:bg-green-400 text-gray-900 font-black shadow-[0_10px_30px_rgba(34,197,94,0.3)] hover:scale-105 active:scale-95 transition-all text-base".to_string()
                            >
                                {move || if loading.get() {
                                    view! {
                                        <div class="flex items-center gap-2">
                                            <div class="w-4 h-4 border-2 border-gray-900/30 border-t-gray-900 rounded-full animate-spin"></div>
                                            "Generando..."
                                        </div>
                                    }.into_any()
                                } else {
                                    "Crear Nuevo Plan".into_any()
                                }}
                            </Button>

                            <div class="flex -space-x-3 items-center ml-2">
                                <div class="w-10 h-10 rounded-full border-2 border-gray-900 bg-gray-800 flex items-center justify-center text-xs font-bold text-gray-400">"JS"</div>
                                <div class="w-10 h-10 rounded-full border-2 border-gray-900 bg-gray-800 flex items-center justify-center text-xs font-bold text-gray-400">"MP"</div>
                                <div class="w-10 h-10 rounded-full border-2 border-gray-900 bg-gray-800 flex items-center justify-center text-xs font-bold text-gray-400">"RA"</div>
                                <div class="pl-6 text-sm font-bold text-gray-500 uppercase tracking-tighter">"+1k usuarios"</div>
                            </div>
                        </div>
                    </div>

                    <div class="hidden lg:block w-72 h-72 relative">
                        <div class="absolute inset-0 glass rounded-3xl rotate-6 group-hover:rotate-12 transition-transform duration-500"></div>
                        <div class="absolute inset-0 bg-green-500/20 rounded-3xl -rotate-3 group-hover:-rotate-6 transition-transform duration-500"></div>
                        <div class="absolute inset-0 flex items-center justify-center text-8xl">
                            "🥗"
                        </div>
                    </div>
                </div>
            </div>

            // Search & Filter Section
            <div class="grid grid-cols-1 lg:grid-cols-4 gap-8 mb-12">
                <div class="lg:col-span-3">
                    <div class="relative group">
                        <div class="absolute inset-0 bg-green-500/10 blur-xl opacity-0 group-focus-within:opacity-100 transition-opacity"></div>
                        <input
                            type="text"
                            placeholder="Buscar en tus planes (ej: Pollo, Lentejas...)"
                            class="relative w-full bg-gray-900/50 backdrop-blur-xl border border-gray-800 rounded-2xl px-14 py-5 text-white focus:ring-2 focus:ring-green-500/30 focus:border-green-500/50 outline-none transition-all placeholder:text-gray-500 text-lg font-medium shadow-2xl"
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_filters.update(|f| f.query = if val.is_empty() { None } else { Some(val) });
                            }
                        />
                        <svg class="w-6 h-6 text-gray-500 absolute left-5 top-1/2 -translate-y-1/2 group-focus-within:text-green-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    </div>
                </div>

                <div class="flex items-center gap-3">
                    <button
                        class=move || format!("flex-1 h-full px-6 py-4 rounded-2xl border transition-all flex items-center justify-center gap-3 font-black uppercase tracking-widest text-xs {}",
                            if filters.get().only_favorites { "glass border-red-500/30 text-red-500 shadow-[0_0_20px_rgba(239,68,68,0.1)]" } else { "glass border-gray-800 text-gray-400 hover:border-gray-700 hover:text-white" })
                        on:click=move |_| set_filters.update(|f| f.only_favorites = !f.only_favorites)
                    >
                        <svg class="w-5 h-5" fill=move || if filters.get().only_favorites { "currentColor" } else { "none" } stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                        </svg>
                        "Favoritos"
                    </button>
                </div>
            </div>

            // Plan List Section
            <div class="space-y-8">
                <div class="flex items-end justify-between px-2">
                    <div>
                        <h2 class="text-3xl font-black text-white tracking-tighter leading-none mb-2">"MIS PLANES"</h2>
                        <div class="h-1.5 w-12 bg-green-500 rounded-full"></div>
                    </div>
                    <span class="bg-gray-800/50 px-4 py-1.5 rounded-full text-[10px] text-gray-500 font-black uppercase tracking-widest border border-gray-700/50">
                        {move || format!("{} encontrados", plans.get().len())}
                    </span>
                </div>

                <div class="grid grid-cols-1 gap-8">
                    <PlanList plans=plans />
                </div>
            </div>
        </div>
    }
}
