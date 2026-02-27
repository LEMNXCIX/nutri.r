use crate::tauri_bridge::{
get_ingredient_stats, toggle_ingredient_exclusion, IngredientStats};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

#[component]
pub fn Ingredients() -> impl IntoView {
    // State signals
    let (ingredients_stats, set_ingredients_stats) = signal(Vec::<IngredientStats>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(String::new());
    let (search_query, set_search_query) = signal(String::new());

    // Fetch data when component mounts
    spawn_local(async move {
        set_loading.set(true);
        match get_ingredient_stats().await {
            Ok(stats) => {
                set_ingredients_stats.set(stats);
                set_loading.set(false);
            }
            Err(e) => {
                set_error.set(e);
                set_loading.set(false);
            }
        }
    });

    let toggle_exclusion = move |ingredient_name: String| {
        // Optimistic UI update
        set_ingredients_stats.update(|stats| {
            for stat in stats.iter_mut() {
                if stat.name == ingredient_name {
                    stat.is_excluded = !stat.is_excluded;
                    break;
                }
            }
        });

        // Call backend
        let name_clone = ingredient_name.clone();
        spawn_local(async move {
            if let Err(e) = toggle_ingredient_exclusion(name_clone).await {
                log!("Error al cambiar estado de ingrediente: {}", e);
            }
        });
    };

    let filtered_ingredients = move || {
        let query = search_query.get().to_lowercase();
        ingredients_stats.get().into_iter()
            .filter(|s| s.name.to_lowercase().contains(&query))
            .collect::<Vec<_>>()
    };

    view! {
        <div class="bg-white min-h-screen font-sans text-neutral-950 pb-32 animate-in fade-in duration-500">
            // -- HEADER --
            <header class="flex items-center justify-between px-6 py-8 sticky top-0 bg-white z-40 border-b border-neutral-100">
                <A href="/" attr:class="flex items-center gap-4">
                    <span class="material-symbols-outlined">"arrow_back"</span>
                </A>
                <div class="text-[10px] font-bold tracking-[0.2em] uppercase">"Library / Ingredients"</div>
                <div class="w-8 h-8 flex items-center justify-center">
                    <span class="material-symbols-outlined">"filter_list"</span>
                </div>
            </header>

            // -- SEARCH & TITLE --
            <section class="px-6 py-10">
                <h1 class="text-5xl font-extrabold uppercase leading-[0.9] tracking-tighter mb-8">
                    "Select" <br/> "Ingredients"
                </h1>
                <div class="relative">
                    <input
                        class="w-full border border-black px-4 py-4 text-xs font-bold tracking-widest uppercase placeholder:text-neutral-300 focus:ring-0 focus:border-black outline-none rounded-none"
                        placeholder="SEARCH DATABASE..."
                        type="text"
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        prop:value=search_query
                    />
                    <div class="absolute right-4 top-1/2 -translate-y-1/2">
                        <span class="material-symbols-outlined text-neutral-400">"search"</span>
                    </div>
                </div>
            </section>

            // -- CATEGORIES --
            <section class="px-6 mb-8 overflow-x-auto whitespace-nowrap scrollbar-hide">
                <div class="flex gap-6">
                    <span class="text-[10px] font-bold uppercase tracking-widest border-b-2 border-black pb-1">"All"</span>
                    <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-400 pb-1">"Proteins"</span>
                    <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-400 pb-1">"Produce"</span>
                    <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-400 pb-1">"Grains"</span>
                </div>
            </section>

            // -- LIST --
            <section class="px-6 space-y-10">
                <Suspense fallback=move || view! { <div class="py-20 text-center uppercase tracking-widest text-[10px] animate-pulse">"Loading Database..."</div> }>
                    {move || {
                        if loading.get() {
                            return ().into_any();
                        }

                        if !error.get().is_empty() {
                            return view! { <div class="p-6 brutalist-border bg-red-50 text-red-500 uppercase font-bold text-[10px]">{error.get()}</div> }.into_any();
                        }

                        let stats = filtered_ingredients();
                        if stats.is_empty() {
                            return view! { <div class="py-20 text-center text-neutral-400 uppercase tracking-widest text-[10px]">"No matching items"</div> }.into_any();
                        }

                        stats.into_iter().map(|stat| {
                            let name = stat.name.clone();
                            let is_excluded = stat.is_excluded;
                            let name_for_click = name.clone();
                            
                            view! {
                                <div class=move || format!("flex items-center justify-between transition-opacity {}", if is_excluded { "opacity-40" } else { "" })>
                                    <div class="flex flex-col gap-1">
                                        <div class="flex items-center gap-2">
                                            <h3 class=move || format!("text-2xl font-light tracking-tighter uppercase {}", if is_excluded { "strikethrough" } else { "" })>
                                                {name.clone()}
                                            </h3>
                                            {if is_excluded {
                                                view! { <span class="text-[8px] px-1 border border-neutral-400 font-bold uppercase tracking-tighter">"Restricted"</span> }.into_any()
                                            } else {
                                                ().into_any()
                                            }}
                                        </div>
                                        <span class="text-[9px] font-bold uppercase tracking-widest text-neutral-400">
                                            {if is_excluded { "Disabled" } else { "Active Entry" }} " / Count: " {stat.count}
                                        </span>
                                    </div>
                                    <div class="flex items-center gap-8">
                                        <button
                                            on:click=move |_| toggle_exclusion(name_for_click.clone())
                                            class=move || format!("p-2 transition-colors {}", if is_excluded { "bg-neutral-100" } else { "bg-accent" })
                                        >
                                            <span class="material-symbols-outlined text-neutral-950">
                                                {if is_excluded { "lock" } else { "add" }}
                                            </span>
                                        </button>
                                    </div>
                                </div>
                                <div class="hairline-divider"></div>
                            }
                        }).collect::<Vec<_>>().into_any()
                    }}
                </Suspense>
            </section>

            // -- FOOTER SELECTION STATUS --
            <div class="fixed bottom-0 left-0 right-0 bg-white border-t border-neutral-100 px-6 py-8 flex justify-between items-center z-50">
                <div class="flex flex-col">
                    <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-400">"Database Status"</span>
                    <span class="text-lg font-light tracking-tighter uppercase">
                        {move || {
                            let total = ingredients_stats.get().len();
                            let active = ingredients_stats.get().iter().filter(|s| !s.is_excluded).count();
                            format!("{} / {} Items Active", active, total)
                        }}
                    </span>
                </div>
                <A href="/" attr:class="bg-neutral-950 text-white px-8 py-3 text-[10px] font-bold uppercase tracking-[0.2em]">
                    "Review Plans"
                </A>
            </div>
        </div>
    }.into_any()
}
