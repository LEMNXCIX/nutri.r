use crate::components::ui::{Button, Card, Loading};
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

    let shopping_resource = LocalResource::new(move || {
        let id = plan_id();
        async move { get_shopping_list(&id).await }
    });

    let (generating, set_generating) = signal(false);

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
        spawn_local(async move {
            if let Err(e) = toggle_shopping_item(&id, &item_name, checked).await {
                log!("Error toggling item: {}", e);
            }
        });
    };

    view! {
        <div class="p-4 md:p-6 max-w-4xl mx-auto font-sans text-gray-900 animate-in fade-in duration-500">
            <header class="mb-8 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div class="flex items-center gap-4">
                     <A href={move || format!("/plan/{}", plan_id())} attr:class="p-2 -ml-2 rounded-full hover:bg-gray-100 text-gray-400 hover:text-black transition-colors">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                        </svg>
                    </A>
                    <div>
                        <h2 class="text-3xl font-black text-black">"Lista de Compras"</h2>
                        <p class="text-gray-500 font-medium">"Ingredientes necesarios para tu plan nutricional."</p>
                    </div>
                </div>

                <Button
                    on_click=Callback::new(on_generate)
                    disabled=generating
                    class="bg-black hover:bg-gray-800 text-white px-6 py-2 rounded-xl flex items-center gap-2 transition-all shadow-lg shadow-gray-200"
                >
                    {move || if generating.get() {
                        view! { <Loading size="w-4 h-4" /> }.into_any()
                    } else {
                        view! {
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                            </svg>
                            <span>"Actualizar Lista"</span>
                        }.into_any()
                    }}
                </Button>
            </header>

            <Suspense fallback=move || view! { <div class="flex justify-center p-12"><Loading /></div> }>
                {move || {
                    match shopping_resource.get() {
                        Some(Ok(Some(list))) => {
                            let categories = group_by_category(list.items);
                            view! {
                                <div class="space-y-6">
                                    {categories.into_iter().map(|(cat, items)| {
                                        view! {
                                            <div>
                                                <h3 class="text-xs font-black text-gray-400 uppercase tracking-widest mb-3 px-1 border-b border-gray-100 pb-1">
                                                    {cat}
                                                </h3>
                                                <Card class="overflow-hidden border border-gray-200 bg-white shadow-sm">
                                                    <div class="divide-y divide-gray-100">
                                                        {items.into_iter().map(|item| {
                                                            let name = item.name.clone();
                                                            let (checked, set_checked) = signal(item.checked);
                                                            view! {
                                                                <div class="flex items-center gap-4 p-4 hover:bg-gray-50 transition-colors group">
                                                                    <input
                                                                        type="checkbox"
                                                                        checked=checked
                                                                        on:change=move |ev| {
                                                                            let val = event_target_checked(&ev);
                                                                            set_checked.set(val);
                                                                            on_toggle(name.clone(), val);
                                                                        }
                                                                        class="w-5 h-5 rounded border-gray-300 text-black focus:ring-black transition-all cursor-pointer"
                                                                    />
                                                                    <div class="flex-1">
                                                                        <span class=move || if checked.get() { "text-gray-400 line-through transition-all font-medium" } else { "text-gray-900 font-medium transition-all" }>
                                                                            {item.name.clone()}
                                                                        </span>
                                                                        {item.quantity.map(|q| view! {
                                                                            <span class="ml-2 text-xs text-gray-500 font-bold bg-gray-100 px-2 py-0.5 rounded-md">
                                                                                {q}
                                                                            </span>
                                                                        })}
                                                                    </div>
                                                                </div>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </Card>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        }
                        Some(Ok(None)) => {
                            view! {
                                <div class="text-center p-16 bg-gray-50 rounded-3xl border border-dashed border-gray-200">
                                    <div class="inline-flex items-center justify-center w-20 h-20 rounded-full bg-white border border-gray-100 mb-6 text-gray-400 shadow-sm">
                                        <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                                        </svg>
                                    </div>
                                    <h3 class="text-xl font-bold text-gray-900 mb-2">"No hay lista generada"</h3>
                                    <p class="text-gray-500 max-w-xs mx-auto mb-8 text-sm">"Haz clic en el botón de arriba para extraer los ingredientes de este plan automáticamente."</p>
                                    <Button on_click=Callback::new(on_generate) class="bg-black hover:bg-gray-800 text-white px-8 h-12 rounded-xl border border-transparent shadow-lg shadow-gray-200">
                                        "Generar ahora"
                                    </Button>
                                </div>
                            }.into_any()
                        }
                        Some(Err(e)) => {
                            view! {
                                <Card class="p-6 border-red-100 bg-red-50 text-red-600">
                                    <div class="flex items-center gap-3">
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <p class="font-medium text-sm">{format!("Error: {}", e)}</p>
                                    </div>
                                </Card>
                            }.into_any()
                        }
                        None => view! { <div class="flex justify-center p-12"><Loading /></div> }.into_any()
                    }
                }}
            </Suspense>
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
