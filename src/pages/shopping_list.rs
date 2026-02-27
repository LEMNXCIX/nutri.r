use crate::components::ui::{Button, Loading};
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
        <div class="bg-[#FAFAFA] min-h-screen font-sans text-[#171717] pb-32">
            <header class="bg-white border-b border-gray-100 pb-10 pt-12 px-4 shadow-sm relative overflow-hidden">
                <div class="absolute top-0 right-0 w-64 h-64 bg-[#D4AF37]/5 -mr-32 -mt-32 rounded-full blur-3xl"></div>
                
                <div class="max-w-4xl mx-auto relative z-10">
                    <div class="flex flex-col md:flex-row md:items-end justify-between gap-8">
                        <div class="space-y-6">
                             <A href={move || format!("/plan/{}", plan_id())} attr:class="inline-flex items-center text-[10px] font-black text-gray-400 hover:text-black tracking-[0.2em] transition-all group uppercase">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-2 group-hover:-translate-x-1 transition-transform" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z" clip-rule="evenodd" />
                                </svg>
                                "VOLVER AL PLAN"
                             </A>
                             <div class="space-y-2">
                                <div class="flex items-center gap-3">
                                    <span class="h-px w-8 bg-[#D4AF37]"></span>
                                    <span class="text-[10px] font-black text-[#D4AF37] tracking-[0.3em] uppercase">"Gestión de Insumos"</span>
                                </div>
                                <h2 class="text-4xl md:text-5xl font-black text-black tracking-tighter leading-none">
                                    "LISTA DE COMPRAS"
                                </h2>
                             </div>
                        </div>

                        <Button
                            on_click=Callback::new(on_generate)
                            disabled=generating
                            class="bg-black hover:bg-[#D4AF37] text-white px-8 py-4 rounded-2xl flex items-center gap-3 transition-all shadow-xl shadow-black/10 text-[11px] font-black tracking-widest uppercase"
                        >
                            {move || if generating.get() {
                                view! { <Loading size="w-4 h-4" /> }.into_any()
                            } else {
                                view! {
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                                    </svg>
                                    <span>"ACTUALIZAR"</span>
                                }.into_any()
                            }}
                        </Button>
                    </div>

                    // PROGRESS BAR - New Mobile Utility
                    <Suspense fallback=move || ()>
                        {move || {
                            if let Some(Ok(Some(list))) = shopping_resource.get() {
                                let total = list.items.len();
                                let checked = list.items.iter().filter(|i| i.checked).count();
                                let pct = if total > 0 { (checked as f32 / total as f32) * 100.0 } else { 0.0 };
                                view! {
                                    <div class="mt-12 space-y-3">
                                        <div class="flex justify-between items-end px-1">
                                            <span class="text-[9px] font-black text-gray-400 uppercase tracking-widest">"Progreso de Compra"</span>
                                            <span class="text-xs font-black text-black">{checked} " / " {total}</span>
                                        </div>
                                        <div class="h-2 w-full bg-gray-50 rounded-full overflow-hidden border border-gray-100">
                                            <div 
                                                class="h-full bg-black rounded-full transition-all duration-700 ease-out"
                                                style=format!("width: {}%", pct)
                                            ></div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else { ().into_any() }
                        }}
                    </Suspense>
                </div>
            </header>

            <div class="max-w-4xl mx-auto px-4 mt-12">
                <Suspense fallback=move || view! { 
                    <div class="space-y-6">
                        <div class="h-32 bg-white rounded-3xl animate-pulse"></div>
                        <div class="h-32 bg-white rounded-3xl animate-pulse"></div>
                    </div>
                }>
                    {move || {
                        match shopping_resource.get() {
                            Some(Ok(Some(list))) => {
                                let categories = group_by_category(list.items);
                                view! {
                                    <div class="space-y-12">
                                        {categories.into_iter().map(|(cat, items)| {
                                            view! {
                                                <section class="space-y-5">
                                                    <h3 class="text-[10px] font-black text-gray-400 uppercase tracking-[0.3em] px-2 flex items-center gap-3">
                                                        <span class="w-1.5 h-1.5 bg-[#D4AF37] rounded-full"></span>
                                                        {cat}
                                                    </h3>
                                                    <div class="bg-white rounded-[2rem] border border-gray-100 shadow-xl shadow-black/5 divide-y divide-gray-50 overflow-hidden">
                                                        {items.into_iter().map(|item| {
                                                            let name = item.name.clone();
                                                            let (checked, set_checked) = signal(item.checked);
                                                            view! {
                                                                <label class="flex items-center gap-5 p-6 md:p-8 hover:bg-gray-50 transition-all group cursor-pointer active:scale-[0.99]">
                                                                    // Custom Tactical Checkbox
                                                                    <div class="relative flex items-center justify-center shrink-0">
                                                                        <input
                                                                            type="checkbox"
                                                                            checked=checked
                                                                            on:change=move |ev| {
                                                                                let val = event_target_checked(&ev);
                                                                                set_checked.set(val);
                                                                                on_toggle(name.clone(), val);
                                                                            }
                                                                            class="peer absolute opacity-0 w-full h-full cursor-pointer z-10"
                                                                        />
                                                                        <div class=move || format!("w-8 h-8 rounded-xl border-2 transition-all flex items-center justify-center {}",
                                                                            if checked.get() { "bg-black border-black shadow-lg" } 
                                                                            else { "bg-white border-gray-200 group-hover:border-[#D4AF37]" }
                                                                        )>
                                                                            <svg class=move || format!("w-4 h-4 transition-all {}", if checked.get() { "text-[#D4AF37] scale-100" } else { "text-transparent scale-50" }) fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="4">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                                                            </svg>
                                                                        </div>
                                                                    </div>

                                                                    <div class="flex-1 min-w-0">
                                                                        <div class="flex flex-wrap items-baseline gap-2">
                                                                            <span class=move || format!("text-base md:text-lg transition-all duration-500 {}", 
                                                                                if checked.get() { "text-gray-300 line-through italic" } else { "text-black font-black tracking-tight" }
                                                                            )>
                                                                                {item.name.clone()}
                                                                            </span>
                                                                            {item.quantity.map(|q| view! {
                                                                                <span class=move || format!("text-[10px] font-black px-2.5 py-1 rounded-lg transition-all {}",
                                                                                    if checked.get() { "bg-gray-50 text-gray-300" } else { "bg-gray-100 text-gray-500" }
                                                                                )>
                                                                                    {q}
                                                                                </span>
                                                                            })}
                                                                        </div>
                                                                    </div>
                                                                </label>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </section>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                            Some(Ok(None)) => {
                                view! {
                                    <div class="text-center py-32 px-8 bg-white rounded-[3rem] border border-gray-100 shadow-xl shadow-black/5">
                                        <div class="w-24 h-24 bg-gray-50 rounded-full flex items-center justify-center mx-auto mb-8 text-gray-300">
                                            <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                                            </svg>
                                        </div>
                                        <h3 class="text-2xl font-black text-black tracking-tighter mb-4">"SIN LISTA GENERADA"</h3>
                                        <p class="text-gray-500 text-sm max-w-xs mx-auto mb-10 leading-relaxed font-medium">"Extraeremos todos los ingredientes de tu plan semanal automáticamente por ti."</p>
                                        <Button on_click=Callback::new(on_generate) class="bg-black hover:bg-[#D4AF37] text-white px-10 py-5 rounded-[1.5rem] font-black text-[11px] tracking-[0.2em] uppercase shadow-2xl shadow-black/20">
                                            "GENERAR LISTA AHORA"
                                        </Button>
                                    </div>
                                }.into_any()
                            }
                            _ => view! { <div class="flex justify-center p-20"><Loading /></div> }.into_any()
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
