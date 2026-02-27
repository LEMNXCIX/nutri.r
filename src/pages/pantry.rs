use crate::components::features::PantryItemCard;
use crate::components::ui::Input;
use crate::tauri_bridge::{
    add_pantry_item, delete_pantry_item, get_pantry_items, update_pantry_item, PantryItem,
};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Pantry() -> impl IntoView {
    let (items_resource, set_items_resource) = signal(0); // Trigger for resource

    let pantry_resource = LocalResource::new(move || {
        let _ = items_resource.get();
        async move { get_pantry_items().await.unwrap_or_default() }
    });

    let (new_item_name, set_new_item_name) = signal(String::new());
    let (new_item_qty, set_new_item_qty) = signal(1.0);
    let (new_item_unit, set_new_item_unit) = signal("kg".to_string());
    let (new_item_cat, set_new_item_cat) = signal("Despensa".to_string());
    let (new_item_exp, set_new_item_exp) = signal(String::new());
    let (show_add_form, set_show_add_form) = signal(false);
    let (item_to_edit, set_item_to_edit) = signal(Option::<PantryItem>::None);

    let on_add_item = move |_| {
        let name = new_item_name.get();
        if name.is_empty() {
            return;
        }

        let id = if let Some(ref edit) = item_to_edit.get() {
            edit.id.clone()
        } else {
            name.to_lowercase().replace(' ', "-")
        };

        let item = PantryItem {
            id,
            name,
            quantity: new_item_qty.get(),
            unit: new_item_unit.get(),
            category: new_item_cat.get(),
            expiration_date: {
                let exp = new_item_exp.get();
                if exp.is_empty() {
                    None
                } else {
                    Some(exp)
                }
            },
        };

        let is_edit = item_to_edit.get().is_some();

        spawn_local(async move {
            let res = if is_edit {
                update_pantry_item(item).await
            } else {
                add_pantry_item(item).await
            };

            match res {
                Ok(_) => {
                    set_items_resource.update(|n| *n += 1);
                    set_new_item_name.set(String::new());
                    set_item_to_edit.set(None);
                    set_show_add_form.set(false);
                }
                Err(e) => log!("Error saving item: {}", e),
            }
        });
    };

    let on_delete = Callback::new(move |id: String| {
        spawn_local(async move {
            match delete_pantry_item(id).await {
                Ok(_) => set_items_resource.update(|n| *n += 1),
                Err(e) => log!("Error deleting item: {}", e),
            }
        });
    });

    let on_update_qty = Callback::new(move |(id, new_qty): (String, f32)| {
        spawn_local(async move {
            let mut items = get_pantry_items().await.unwrap_or_default();
            if let Some(item) = items.iter_mut().find(|i| i.id == id) {
                item.quantity = new_qty;
                match update_pantry_item(item.clone()).await {
                    Ok(_) => set_items_resource.update(|n| *n += 1),
                    Err(e) => log!("Error updating qty: {}", e),
                }
            }
        });
    });

    let h_update_name = Callback::new(move |v| set_new_item_name.set(v));

    let on_edit = Callback::new(move |item: PantryItem| {
        set_new_item_name.set(item.name.clone());
        set_new_item_qty.set(item.quantity);
        set_new_item_unit.set(item.unit.clone());
        set_new_item_cat.set(item.category.clone());
        set_new_item_exp.set(item.expiration_date.clone().unwrap_or_default());
        set_item_to_edit.set(Some(item));
        set_show_add_form.set(true);
    });

    view! {
        <div class="w-full font-sans pb-32">
            // Header - Brutalist & Bold
            <header class="bg-white dark:bg-neutral-900 border-b brutalist-border dark:border-neutral-800 py-12 px-6 mb-12">
                <div class="max-w-7xl mx-auto flex flex-col md:flex-row md:items-end justify-between gap-8">
                    <div class="space-y-4">
                        <div class="flex items-center gap-3">
                            <span class="h-[1px] w-8 bg-accent"></span>
                            <span class="text-[10px] font-black text-neutral-400 dark:text-neutral-500 tracking-[0.4em] uppercase">"Stock Mastery"</span>
                        </div>
                        <h2 class="text-6xl md:text-8xl font-black text-neutral-950 dark:text-white tracking-tighter leading-[0.8] uppercase">
                            "Despensa"
                        </h2>
                        <div class="hairline-divider dark:bg-neutral-700 w-12 mt-6"></div>
                    </div>

                    <button
                        on:click=move |_| {
                            if show_add_form.get() {
                                set_show_add_form.set(false);
                                set_item_to_edit.set(None);
                                set_new_item_name.set(String::new());
                            } else {
                                set_show_add_form.set(true);
                            }
                        }
                        class=format!("px-10 py-5 brutalist-border transition-all active:scale-95 text-[11px] font-black tracking-widest uppercase {}",
                            if show_add_form.get() { "bg-white text-red-500 hover:bg-neutral-50" } else { "bg-neutral-950 text-white hover:bg-accent hover:text-neutral-950" })
                    >
                        {move || if show_add_form.get() { "CANCELAR" } else { "NUEVO INGREDIENTE" }}
                    </button>
                </div>
            </header>

            <div class="max-w-7xl mx-auto px-6">
                {move || if show_add_form.get() {
                    view! {
                        <div class="mb-24 animate-in fade-in slide-in-from-top-4 duration-500">
                            <div class="p-10 bg-white dark:bg-neutral-900 brutalist-border dark:border-neutral-700 shadow-brutalist relative overflow-hidden">
                                <div class="absolute top-0 left-0 w-full h-1 bg-accent"></div>

                                <h3 class="text-[10px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-[0.3em] mb-12 flex items-center gap-3">
                                    <span class="material-symbols-outlined !text-[18px]">"inventory_2"</span>
                                    {move || if item_to_edit.get().is_some() { "MODIFICAR REGISTRO" } else { "ALTA DE INSUMO" }}
                                </h3>

                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                                    <div class="space-y-4">
                                        <label class="block text-[9px] font-black uppercase tracking-[0.3em] text-neutral-400 dark:text-neutral-500 pl-1">"Identificador"</label>
                                        <Input
                                            placeholder="Arroz, Pollo, Leche..."
                                            value=new_item_name
                                            on_input=h_update_name
                                            class="bg-white dark:bg-neutral-800 dark:text-white dark:border-neutral-700 brutalist-border focus:border-accent p-4 font-bold text-sm uppercase"
                                        />
                                    </div>

                                    <div class="space-y-4">
                                        <label class="block text-[9px] font-black uppercase tracking-[0.3em] text-neutral-400 dark:text-neutral-500 pl-1">"Escala y Unidad"</label>
                                        <div class="flex gap-0 brutalist-border dark:border-neutral-700 bg-white dark:bg-neutral-800">
                                            <input
                                                type="number"
                                                step="0.1"
                                                class="w-24 bg-white dark:bg-neutral-800 text-neutral-950 dark:text-white px-4 py-4 outline-none border-r border-neutral-950 dark:border-neutral-600 font-black text-sm"
                                                on:input=move |ev| {
                                                    if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                                        set_new_item_qty.set(val);
                                                    }
                                                }
                                                prop:value=new_item_qty
                                            />
                                            <select
                                                class="flex-1 bg-transparent dark:text-white px-4 py-4 text-neutral-950 outline-none font-black text-[10px] uppercase tracking-widest cursor-pointer"
                                                on:change=move |ev| set_new_item_unit.set(event_target_value(&ev))
                                                prop:value=new_item_unit
                                            >
                                                <option value="kg">"kg"</option>
                                                <option value="g">"g"</option>
                                                <option value="L">"L"</option>
                                                <option value="un">"un"</option>
                                            </select>
                                        </div>
                                    </div>

                                    <div class="space-y-4">
                                        <label class="block text-[9px] font-black uppercase tracking-[0.3em] text-neutral-400 dark:text-neutral-500 pl-1">"Clasificación"</label>
                                        <select
                                            class="w-full bg-white dark:bg-neutral-800 text-neutral-950 dark:text-white px-5 py-4 brutalist-border dark:border-neutral-700 focus:border-accent text-neutral-950 outline-none transition-all font-black text-[10px] uppercase tracking-[0.2em] cursor-pointer"
                                            on:change=move |ev| set_new_item_cat.set(event_target_value(&ev))
                                            prop:value=new_item_cat
                                        >
                                            <option value="Despensa">"Despensa"</option>
                                            <option value="Refrigerados">"Refrigerados"</option>
                                            <option value="Congelados">"Congelados"</option>
                                            <option value="Frutas/Verduras">"Frescos"</option>
                                        </select>
                                    </div>

                                    <div class="space-y-4">
                                        <label class="block text-[9px] font-black uppercase tracking-[0.3em] text-neutral-400 dark:text-neutral-500 pl-1">"Vencimiento"</label>
                                        <input
                                            type="date"
                                            class="w-full bg-white dark:bg-neutral-800 text-neutral-950 dark:text-white px-5 py-4 brutalist-border dark:border-neutral-700 focus:border-accent outline-none transition-all font-black text-[10px] uppercase tracking-[0.2em] cursor-pointer"
                                            on:input=move |ev| set_new_item_exp.set(event_target_value(&ev))
                                            prop:value=new_item_exp
                                        />
                                    </div>
                                </div>

                                <div class="mt-12 flex justify-end">
                                    <button on:click=move |_| on_add_item(()) class="px-12 py-5 brutalist-border bg-neutral-950 dark:bg-white text-white dark:text-black hover:bg-accent dark:hover:bg-accent hover:text-neutral-950 transition-all text-[11px] font-black tracking-[0.3em] uppercase">
                                        {move || if item_to_edit.get().is_some() { "ACTUALIZAR" } else { "GUARDAR" }}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }}

                <Suspense fallback=move || view! {
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
                        {(0..8).map(|_| view! { <div class="h-64 brutalist-border bg-neutral-50 animate-pulse"></div> }).collect_view()}
                    </div>
                }>
                    {move || {
                        let items = pantry_resource.get().unwrap_or_default();
                        if items.is_empty() {
                            view! {
                                <div class="bg-white dark:bg-neutral-900 brutalist-border dark:border-neutral-700 py-32 px-12 text-center shadow-brutalist flex flex-col items-center gap-8 max-w-2xl mx-auto">
                                    <div class="w-24 h-24 brutalist-border dark:border-neutral-700 bg-neutral-50 dark:bg-neutral-800 flex items-center justify-center text-neutral-200 dark:text-neutral-600">
                                        <span class="material-symbols-outlined !text-[48px]">"inventory_2"</span>
                                    </div>
                                    <div class="space-y-4">
                                        <h3 class="text-4xl font-black text-neutral-950 dark:text-white tracking-tighter uppercase">"Inventario Vacío"</h3>
                                        <p class="text-neutral-400 dark:text-neutral-500 font-bold text-[10px] max-w-xs mx-auto uppercase tracking-[0.4em] leading-relaxed">"Registra tus primeros insumos para que la IA pueda considerarlos en tus planes."</p>
                                    </div>
                                    <button on:click=move |_| set_show_add_form.set(true) class="bg-neutral-950 dark:bg-white text-white dark:text-black px-12 py-5 brutalist-border font-black text-[11px] tracking-widest uppercase hover:bg-accent dark:hover:bg-accent hover:text-neutral-950 transition-colors">"AGREGAR AHORA"</button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8 animate-in fade-in duration-700">
                                    {items.into_iter().map(|item| {
                                        view! {
                                            <PantryItemCard
                                                item=item
                                                on_delete=on_delete.clone()
                                                on_update_qty=on_update_qty.clone()
                                                on_edit=on_edit.clone()
                                            />
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
