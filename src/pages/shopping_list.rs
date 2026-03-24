use crate::components::ui::Loading;
use crate::tauri_bridge::{generate_shopping_list, get_shopping_list, toggle_shopping_item};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ShoppingList() -> impl IntoView {
    let params = use_params_map();
    let plan_id = move || params.read().get("id").unwrap_or_default();

    let (items, set_items) = signal(Vec::<crate::tauri_bridge::ShoppingItem>::new());
    let (generating, set_generating) = signal(false);

    let shopping_resource = LocalResource::new(move || {
        let id = plan_id();
        async move { get_shopping_list(&id).await }
    });

    Effect::new(move |_| {
        if let Some(Ok(Some(list))) = shopping_resource.get() {
            set_items.set(list.items);
        }
    });

    let on_generate = move |_| {
        let id = plan_id();
        set_generating.set(true);
        spawn_local(async move {
            match generate_shopping_list(&id).await {
                Ok(_) => {
                    shopping_resource.refetch();
                }
                Err(e) => {
                    log!("Error generating shopping list: {}", e);
                }
            }
            set_generating.set(false);
        });
    };

    let on_toggle = move |item_name: String, checked: bool| {
        let id = plan_id();
        // Optimistic update
        set_items.update(|list| {
            if let Some(item) = list.iter_mut().find(|i| i.name == item_name) {
                item.checked = checked;
            }
        });

        spawn_local(async move {
            if let Err(e) = toggle_shopping_item(&id, &item_name, checked).await {
                log!("Error toggling item: {}", e);
            }
        });
    };

    let progress_data = move || {
        let current_items = items.get();
        let total = current_items.len();
        let checked = current_items.iter().filter(|i| i.checked).count();
        let pct = if total > 0 {
            (checked as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        (checked, total, pct)
    };

    view! {
        <div class="w-full font-sans pb-32">
            // Header - Editorial Brutalist Style
            <header class="bg-white dark:bg-neutral-900 border-b border-neutral-950 dark:border-neutral-800 pb-16 pt-16 px-6 relative overflow-hidden">
                <div class="max-w-5xl mx-auto relative z-10">
                    <div class="flex flex-col md:flex-row md:items-end justify-between gap-10">
                        <div class="space-y-8">
                            <A
                                href=move || format!("/plan/{}", plan_id())
                                attr:class="inline-flex items-center text-[10px] font-black text-neutral-400 hover:text-neutral-950 dark:hover:text-white tracking-[0.3em] transition-all group uppercase"
                            >
                                <span class="material-symbols-outlined !text-sm mr-2 group-hover:-translate-x-1 transition-transform">
                                    "arrow_back"
                                </span>
                                "Volver al Plan"
                            </A>
                            <div class="space-y-3">
                                <div class="flex items-center gap-3">
                                    <span class="h-px w-10 bg-accent"></span>
                                    <span class="text-[10px] font-black text-accent tracking-[0.4em] uppercase">
                                        "Protocolo de Inventario"
                                    </span>
                                </div>
                                <h2 class="text-[72px] break-words md:text-7xl font-black text-neutral-950 dark:text-white tracking-tighter leading-none uppercase">
                                    "Lista de" <br /> "Compras"
                                </h2>
                            </div>
                        </div>

                        <button
                            on:click=on_generate
                            disabled=generating
                            class="group relative px-10 py-5 bg-neutral-950 dark:bg-white text-white dark:text-neutral-950 border border-neutral-950 font-black text-[12px] tracking-[0.3em] uppercase hover:bg-accent hover:text-neutral-950 transition-all active:translate-x-1 active:translate-y-1 shadow-brutalist"
                        >
                            {move || {
                                if generating.get() {
                                    view! { <Loading size="w-4 h-4" /> }.into_any()
                                } else {
                                    view! {
                                        <div class="flex items-center gap-3">
                                            <span class="material-symbols-outlined !text-lg">"sync"</span>
                                            <span>"Regenerar"</span>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </button>
                    </div>

                    // PROGRESS BAR - Integrated & Reactive
                    <div class="mt-16 space-y-4 max-w-2xl">
                        <div class="flex justify-between items-end px-1">
                            <span class="text-[11px] font-black text-neutral-400 uppercase tracking-widest">
                                "Sincronización"
                            </span>
                            <span class="text-xl font-black text-neutral-950 dark:text-white tabular-nums tracking-tighter">
                                {move || {
                                    let (checked, total, _) = progress_data();
                                    format!("{:02} / {:02}", checked, total)
                                }}
                            </span>
                        </div>
                        <div class="h-3 w-full bg-neutral-100 dark:bg-neutral-800 border border-neutral-950 p-[2px]">
                            <div
                                class="h-full bg-accent transition-all duration-500 ease-out"
                                style:width=move || format!("{}%", progress_data().2)
                            ></div>
                        </div>
                    </div>
                </div>
            </header>

            <div class="max-w-5xl mx-auto px-6 mt-16">
                <Suspense fallback=move || {
                    view! {
                        <div class="space-y-8">
                            <div class="h-48 bg-neutral-50 dark:bg-neutral-900 border border-neutral-100 dark:border-neutral-800 animate-pulse"></div>
                            <div class="h-48 bg-neutral-50 dark:bg-neutral-900 border border-neutral-100 dark:border-neutral-800 animate-pulse"></div>
                        </div>
                    }
                }>
                    {move || {
                        let current_items = items.get();
                        if current_items.is_empty() && shopping_resource.get().is_some() {
                            view! {
                                <div class="text-center py-40 px-8 bg-neutral-50 dark:bg-neutral-900 border border-neutral-950 shadow-brutalist">
                                    <div class="w-24 h-24 border-2 border-dashed border-neutral-200 dark:border-neutral-800 flex items-center justify-center mx-auto mb-10">
                                        <span class="material-symbols-outlined !text-4xl text-neutral-300">"inventory_2"</span>
                                    </div>
                                    <h3 class="text-3xl font-black text-neutral-950 dark:text-white tracking-tighter mb-4 uppercase">"Manifiesto Vacío"</h3>
                                    <p class="text-neutral-500 dark:text-neutral-400 text-[10px] font-bold tracking-widest uppercase max-w-xs mx-auto mb-12 leading-relaxed">
                                        "No se detectaron artículos. Inicia el protocolo para extraer ingredientes de tu plan."
                                    </p>
                                    <button
                                        on:click=on_generate
                                        class="bg-accent text-neutral-950 px-12 py-5 border border-neutral-950 font-black text-[11px] tracking-[0.3em] uppercase hover:bg-neutral-950 hover:text-accent transition-all active:translate-x-1 active:translate-y-1 shadow-brutalist"
                                    >
                                        "Ejecutar Extracción"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            let categories = group_by_category(current_items);
                            view! {
                                <div class="space-y-20">
                                    {categories.into_iter().map(|(cat, cat_items)| {
                                        view! {
                                            <section class="group">
                                                <div class="flex items-center gap-4 mb-8">
                                                    <div class="w-2 h-2 bg-accent"></div>
                                                    <h3 class="text-[11px] font-black text-neutral-400 uppercase tracking-[0.4em] group-hover:text-neutral-950 dark:group-hover:text-white transition-colors">
                                                        {cat}
                                                    </h3>
                                                    <div class="flex-1 h-px bg-neutral-100 dark:bg-neutral-800"></div>
                                                </div>

                                                <div class="grid grid-cols-1 border-t border-l border-neutral-950">
                                                    {cat_items.into_iter().map(|item| {
                                                        let name = item.name.clone();
                                                        let is_checked = item.checked;
                                                        view! {
                                                            <label class="flex items-center gap-6 p-8 border-r border-b border-neutral-950 bg-white dark:bg-neutral-900 hover:bg-neutral-50 dark:hover:bg-neutral-800/50 transition-all cursor-pointer group/item relative overflow-hidden">
                                                                <input
                                                                    type="checkbox"
                                                                    checked=is_checked
                                                                    on:change=move |ev| {
                                                                        let val = event_target_checked(&ev);
                                                                        on_toggle(name.clone(), val);
                                                                    }
                                                                    class="peer absolute opacity-0 w-full h-full cursor-pointer z-10"
                                                                />

                                                                // Custom Brutalist Checkbox
                                                                <div class=move || format!("w-10 h-10 border-2 transition-all flex items-center justify-center shrink-0 {}",
                                                                    if is_checked { "bg-accent border-neutral-950" }
                                                                    else { "bg-white dark:bg-neutral-800 border-neutral-950 group-hover/item:bg-neutral-100" }
                                                                )>
                                                                    <span class=move || format!("material-symbols-outlined !text-xl !font-black transition-all {}",
                                                                        if is_checked { "text-neutral-950 scale-100" } else { "text-transparent scale-50" }
                                                                    )>
                                                                        "check"
                                                                    </span>
                                                                </div>

                                                                <div class="flex-1 min-w-0">
                                                                    <div class="flex flex-wrap items-baseline gap-4">
                                                                        <span class=move || format!("text-xl transition-all duration-300 {}",
                                                                            if is_checked { "text-neutral-300 dark:text-neutral-700 line-through skew-x-[-12deg]" }
                                                                            else { "text-neutral-950 dark:text-white font-black tracking-tighter" }
                                                                        )>
                                                                            {item.name.clone()}
                                                                        </span>
                                                                        {item.quantity.map(|q| view! {
                                                                            <span class=move || format!("text-[10px] font-black px-3 py-1 border transition-all {}",
                                                                                if is_checked { "border-neutral-100 bg-neutral-50 text-neutral-300 dark:border-neutral-800 dark:bg-neutral-900 dark:text-neutral-700" }
                                                                                else { "border-neutral-950 bg-white text-neutral-950 dark:bg-neutral-900 dark:text-neutral-400 group-hover/item:bg-accent" }
                                                                            )>
                                                                                {q}
                                                                            </span>
                                                                        })}
                                                                    </div>
                                                                </div>
                                                            </label>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </section>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

fn group_by_category(
    items: Vec<crate::tauri_bridge::ShoppingItem>,
) -> Vec<(String, Vec<crate::tauri_bridge::ShoppingItem>)> {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    for item in items {
        map.entry(item.category.clone())
            .or_insert_with(Vec::new)
            .push(item);
    }
    map.into_iter().collect()
}
