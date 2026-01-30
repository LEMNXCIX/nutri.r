use crate::components::ui::{Card, Loading};
use crate::tauri_bridge::{get_ingredient_stats, toggle_ingredient_exclusion, IngredientStats};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Ingredients() -> impl IntoView {
    // State signals
    let (ingredients_stats, set_ingredients_stats) = signal(Vec::<IngredientStats>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(String::new());

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

    view! {
        <div class="p-4 md:p-6 max-w-5xl mx-auto">
            <header class="mb-8">
                <h2 class="text-3xl font-bold text-white mb-2">"Ingredientes"</h2>
                <p class="text-gray-400">"Gestiona la frecuencia y exclusión de ingredientes en tus planes."</p>
                <p class="text-xs text-green-400/70 mt-1 uppercase tracking-widest font-bold">
                    "Haz clic para alternar exclusión"
                </p>
            </header>

            {move || {
                if loading.get() {
                    view! { <div class="flex justify-center p-12"><Loading /></div> }.into_any()
                } else if !error.get().is_empty() {
                    view! {
                        <Card>
                            <h3 class="text-xl font-bold text-red-400 mb-2">"Error"</h3>
                            <p class="text-gray-300">
                                {format!("Error al cargar los ingredientes: {}", error.get())}
                            </p>
                        </Card>
                    }.into_any()
                } else {
                    let stats = ingredients_stats.get();
                    if stats.is_empty() {
                        view! {
                            <div class="text-center p-12 bg-gray-800/50 rounded-2xl border border-gray-700">
                                <p class="text-gray-400">"No hay ingredientes registrados aún."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                                {stats.into_iter().map(|stat| {
                                    let name = stat.name.clone();
                                    let count = stat.count;
                                    let is_excluded = stat.is_excluded;
                                    let name_for_click = name.clone();

                                    view! {
                                        <div
                                            on:click=move |_| toggle_exclusion(name_for_click.clone())
                                            class=move || format!(
                                                "cursor-pointer group relative p-4 rounded-xl border transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl {}",
                                                if is_excluded {
                                                    "bg-red-950/20 border-red-900/40 opacity-70 hover:opacity-100"
                                                } else {
                                                    "bg-gray-800 border-gray-700 hover:border-green-500/50 hover:bg-gray-750"
                                                }
                                            )
                                        >
                                            <div class="flex flex-col gap-1">
                                                <div class="flex justify-between items-start">
                                                    <span class=move || format!(
                                                        "text-lg font-bold truncate pr-8 {}",
                                                        if is_excluded { "text-red-300 line-through" } else { "text-white" }
                                                    )>
                                                        {name.clone()}
                                                    </span>
                                                    <span class="flex items-center justify-center w-8 h-8 rounded-lg bg-gray-900/50 text-xs font-mono text-green-400 border border-gray-700">
                                                        {count}
                                                    </span>
                                                </div>

                                                <div class="flex items-center gap-2 mt-2">
                                                    {if is_excluded {
                                                        view! {
                                                            <span class="text-[10px] uppercase tracking-tighter font-black px-2 py-0.5 rounded-full bg-red-900 text-red-100">
                                                                "🚫 Excluido"
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <span class="text-[10px] uppercase tracking-tighter font-black px-2 py-0.5 rounded-full bg-green-900/50 text-green-300">
                                                                "✓ Permitido"
                                                            </span>
                                                        }.into_any()
                                                    }}
                                                </div>
                                            </div>

                                            <div class="absolute top-4 right-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                                <svg class="w-4 h-4 text-gray-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
                                                </svg>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
