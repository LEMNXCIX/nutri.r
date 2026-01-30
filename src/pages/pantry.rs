use crate::components::features::PantryItemCard;
use crate::components::ui::{Button, Card, Input, Loading};
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
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 animate-in fade-in duration-700">
            <header class="mb-12 flex flex-col md:flex-row md:items-end justify-between gap-6">
                <div class="text-center md:text-left">
                    <span class="inline-block px-4 py-1.5 rounded-full bg-blue-500/10 text-blue-400 text-[10px] font-black uppercase tracking-[0.2em] mb-4">
                        "Inventory Management"
                    </span>
                    <h2 class="text-5xl font-black text-white tracking-tighter mb-4 uppercase italic premium-gradient-text">
                        "Tu Despensa"
                    </h2>
                    <div class="h-1.5 w-24 bg-blue-500 rounded-full mb-4 hidden md:block"></div>
                    <p class="text-gray-400 font-medium max-w-2xl uppercase tracking-widest text-[10px] leading-relaxed">
                        "Gestiona tus existencias para optimizar la generación de planes nutricionales."
                    </p>
                </div>

                <Button
                    on_click=Callback::new(move |_| {
                        if show_add_form.get() {
                            set_show_add_form.set(false);
                            set_item_to_edit.set(None);
                            set_new_item_name.set(String::new());
                        } else {
                            set_show_add_form.set(true);
                        }
                    })
                    class=format!("px-8 py-4 rounded-2xl shadow-2xl transition-all active:scale-95 {}", if show_add_form.get() { "bg-red-500/10 text-red-400 border-red-500/20" } else { "shadow-green-500/20" })
                >
                    {move || if show_add_form.get() { "CANCELAR" } else { "NUEVO INGREDIENTE" }}
                </Button>
            </header>

            {move || if show_add_form.get() {
                view! {
                    <div class="mb-12 animate-in fade-in slide-in-from-top-4 duration-500">
                        <Card class="p-10 glass rounded-[3rem] border-white/5 shadow-3xl relative overflow-hidden">
                            <div class="absolute -top-12 -right-12 w-32 h-32 bg-green-500/5 blur-3xl"></div>
                            <h3 class="text-xl font-black text-white mb-8 uppercase italic tracking-tighter flex items-center gap-3">
                                <div class="w-8 h-8 rounded-xl bg-blue-500/10 flex items-center justify-center">
                                    <svg class="w-4 h-4 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" /></svg>
                                </div>
                                {move || if item_to_edit.get().is_some() { "Editar Ingrediente" } else { "Agregar a Despensa" }}
                            </h3>

                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                                <div class="space-y-3">
                                    <label class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Nombre"</label>
                                    <Input
                                        placeholder="Arroz, Pollo, Leche..."
                                        value=new_item_name
                                        on_input=h_update_name
                                    />
                                </div>

                                <div class="space-y-3">
                                    <label class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Cantidad y Unidad"</label>
                                    <div class="flex gap-2">
                                        <input
                                            type="number"
                                            step="0.1"
                                            class="w-24 glass-light px-4 py-3 rounded-xl border-white/5 text-white outline-none focus:ring-2 focus:ring-green-500/50 transition-all font-black text-sm"
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                                    set_new_item_qty.set(val);
                                                }
                                            }
                                            prop:value=new_item_qty
                                        />
                                            <select
                                                class="flex-1 glass-light px-4 py-3 rounded-xl border-white/5 text-gray-300 outline-none focus:ring-2 focus:ring-green-500/50 transition-all font-black text-[10px] uppercase tracking-widest"
                                                on:change=move |ev| set_new_item_unit.set(event_target_value(&ev))
                                                prop:value=new_item_unit
                                            >
                                                <option value="kg">"kg"</option>
                                                <option value="g">"g"</option>
                                                <option value="L">"L"</option>
                                                <option value="ml">"ml"</option>
                                                <option value="un">"un"</option>
                                            </select>
                                    </div>
                                </div>

                                <div class="space-y-3">
                                    <label class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Categoría"</label>
                                    <select
                                        class="w-full glass-light px-4 py-3 rounded-xl border-white/5 text-gray-300 outline-none focus:ring-2 focus:ring-green-500/50 transition-all font-black text-[10px] uppercase tracking-widest"
                                        on:change=move |ev| set_new_item_cat.set(event_target_value(&ev))
                                        prop:value=new_item_cat
                                    >
                                        <option value="Despensa">"Despensa"</option>
                                        <option value="Refrigerados">"Refrigerados"</option>
                                        <option value="Congelados">"Congelados"</option>
                                        <option value="Frutas/Verduras">"Frutas/Verduras"</option>
                                    </select>
                                </div>

                                <div class="space-y-3">
                                    <label class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Vencimiento"</label>
                                    <input
                                        type="date"
                                        class="w-full glass-light px-4 py-2.5 rounded-xl border-white/5 text-gray-400 outline-none focus:ring-2 focus:ring-green-500/50 transition-all font-black text-[10px] uppercase tracking-widest"
                                        on:input=move |ev| set_new_item_exp.set(event_target_value(&ev))
                                        prop:value=new_item_exp
                                    />
                                </div>
                            </div>

                            <div class="mt-10 flex justify-end">
                                <Button on_click=Callback::new(on_add_item) class="px-10 py-5 rounded-[2rem] shadow-2xl shadow-blue-500/20 active:scale-95 transition-all".to_string()>
                                    {move || if item_to_edit.get().is_some() { "Actualizar Cambios" } else { "Guardar Ingrediente" }}
                                </Button>
                            </div>
                        </Card>
                    </div>
                }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}

            <Suspense fallback=move || view! { <div class="flex justify-center py-20"><Loading /></div> }>
                {move || {
                    let items = pantry_resource.get().unwrap_or_default();
                    if items.is_empty() {
                        view! {
                            <div class="glass rounded-[4rem] p-32 text-center border-dashed border-2 border-white/5 opacity-50 flex flex-col items-center gap-6">
                                <div class="w-24 h-24 bg-white/5 rounded-full flex items-center justify-center text-5xl grayscale">"📦"</div>
                                <div class="space-y-2">
                                    <p class="text-gray-300 font-black uppercase tracking-[0.2em] text-lg italic">"Tu despensa está vacía"</p>
                                    <p class="text-[10px] text-gray-500 font-black uppercase tracking-widest">"Agrega ingredientes para empezar a tener un control total"</p>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
                                {items.into_iter().map(|item| {
                                    view! {
                                        <PantryItemCard
                                            item=item
                                            on_delete=on_delete
                                            on_update_qty=on_update_qty
                                            on_edit=on_edit
                                        />
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
