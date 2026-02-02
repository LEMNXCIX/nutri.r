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
        <div class="p-4 md:p-6 max-w-5xl mx-auto font-sans">
            <header class="mb-10">
                <div class="space-y-1">
                    <span class="text-xs font-black text-gray-400 tracking-widest uppercase">"CONFIGURACIÓN"</span>
                    <h2 class="text-4xl font-black text-black tracking-tighter mb-2">"INGREDIENTES"</h2>
                </div>
                <p class="text-gray-500 font-medium max-w-2xl">"Gestiona la frecuencia y exclusión de ingredientes en tus planes. Los ingredientes excluidos no aparecerán en tus futuras generaciones."</p>
            </header>

            {move || {
                if loading.get() {
                    view! { <div class="flex justify-center p-12"><Loading /></div> }.into_any()
                } else if !error.get().is_empty() {
                    view! {
                        <Card class="border-red-100 bg-red-50">
                            <h3 class="text-lg font-bold text-red-600 mb-1">"Error"</h3>
                            <p class="text-red-500 text-sm">
                                {format!("Error al cargar los ingredientes: {}", error.get())}
                            </p>
                        </Card>
                    }.into_any()
                } else {
                    let stats = ingredients_stats.get();
                    if stats.is_empty() {
                        view! {
                            <div class="text-center p-16 bg-gray-50 rounded-[2rem] border border-dashed border-gray-200">
                                <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-white border border-gray-100 mb-4 text-gray-300">
                                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg>
                                </div>
                                <h3 class="text-lg font-bold text-gray-900 mb-1">"Sin ingredientes"</h3>
                                <p class="text-gray-400 text-sm">"No hay ingredientes registrados aún."</p>
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
                                                "cursor-pointer group relative p-5 rounded-2xl border transition-all duration-300 {}",
                                                if is_excluded {
                                                    "bg-gray-50 border-gray-100 hover:border-gray-200"
                                                } else {
                                                    "bg-white border-gray-200 hover:border-black hover:shadow-lg hover:shadow-black/5 hover:-translate-y-1"
                                                }
                                            )
                                        >
                                            <div class="flex flex-col gap-3">
                                                <div class="flex justify-between items-start">
                                                    <span class=move || format!(
                                                        "text-lg font-bold capitalize truncate pr-4 {}",
                                                        if is_excluded { "text-gray-400 line-through decoration-2 decoration-gray-300" } else { "text-gray-900" }
                                                    )>
                                                        {name.clone()}
                                                    </span>
                                                    <span class=format!("flex items-center justify-center min-w-[2rem] h-8 rounded-lg text-xs font-bold border {}",
                                                        if is_excluded { "bg-gray-100 text-gray-400 border-gray-100" } else { "bg-gray-50 text-black border-gray-100" }
                                                    )>
                                                        {count}
                                                    </span>
                                                </div>

                                                <div class="flex items-center justify-between mt-1">
                                                    {if is_excluded {
                                                        view! {
                                                            <span class="inline-flex items-center gap-1.5 text-[10px] uppercase tracking-wider font-black px-2.5 py-1 rounded-md bg-gray-100 text-gray-400">
                                                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" /></svg>
                                                                "Excluido"
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <span class="inline-flex items-center gap-1.5 text-[10px] uppercase tracking-wider font-black px-2.5 py-1 rounded-md bg-white border border-gray-100 text-gray-500 group-hover:text-black group-hover:border-black/10 transition-colors">
                                                                <svg class="w-3 h-3 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                                                "Habilitado"
                                                            </span>
                                                        }.into_any()
                                                    }}

                                                    // Action hint (hidden by default, shown on hover)
                                                    <div class="opacity-0 group-hover:opacity-100 transition-opacity text-[10px] uppercase font-bold text-gray-300">
                                                        {if is_excluded { "Habilitar" } else { "Excluir" }}
                                                    </div>
                                                </div>
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
