use chrono::NaiveDate;
use leptos::prelude::*;

use crate::tauri_bridge::PantryItem;

#[component]
pub fn PantryItemCard(
    item: PantryItem,
    on_delete: Callback<String>,
    on_update_qty: Callback<(String, f32)>,
    on_edit: Callback<PantryItem>,
) -> impl IntoView {
    let item_dt = item.clone();

    let expiration_status = move || {
        if let Some(ref date_str) = item_dt.expiration_date {
            if let Ok(exp_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                let today = chrono::Local::now().date_naive();
                let days_until = (exp_date - today).num_days();

                if days_until < 0 {
                    ("VENCIDO", "text-red-600 bg-red-50 border-red-100")
                } else if days_until <= 7 {
                    (
                        "VENCE PRONTO",
                        "text-orange-600 bg-orange-50 border-orange-100",
                    )
                } else {
                    ("EN ESTADO", "text-gray-600 bg-gray-50 border-gray-200")
                }
            } else {
                ("FECHA INVÁLIDA", "text-gray-400 bg-gray-50 border-gray-200")
            }
        } else {
            (
                "SIN VENCIMIENTO",
                "text-gray-400 bg-gray-50 border-gray-200",
            )
        }
    };

    let (status_text, status_class) = expiration_status();
    let id_del = item.id.clone();
    let id_qty = item.id.clone();

    view! {
        <div class="card p-6 flex flex-col justify-between h-full group relative overflow-hidden transition-all duration-300 hover:shadow-soft-lg">
            <div>
                <div class="flex justify-between items-start mb-4">
                    <div class="flex-1">
                        <span class="text-[10px] font-bold uppercase tracking-wider text-gray-400 mb-1 block">{item.category.clone()}</span>
                        <h3 class="text-lg font-bold text-gray-900 tracking-tight uppercase group-hover:text-black transition-colors">{item.name.clone()}</h3>
                    </div>

                    <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                        <button
                            type="button"
                            on:click={
                                let item = item.clone();
                                move |ev| {
                                    ev.stop_propagation();
                                    on_edit.run(item.clone());
                                }
                            }
                            class="p-2 rounded-lg bg-gray-50 text-gray-500 hover:text-black hover:bg-gray-100 transition-all active:scale-95 border border-gray-200"
                            title="Editar producto"
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                            </svg>
                        </button>

                        <button
                            type="button"
                            on:click={
                                let id = id_del.clone();
                                move |ev| {
                                    ev.stop_propagation();
                                    leptos::logging::log!("Click en eliminar para id: {}", id);
                                    on_delete.run(id.clone());
                                }
                            }
                            class="p-2 rounded-lg bg-gray-50 text-gray-500 hover:text-red-600 hover:bg-red-50 transition-all active:scale-95 border border-gray-200"
                            title="Eliminar producto"
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="flex items-center gap-4 mb-6">
                    <div class="flex-1 bg-gray-50 rounded-xl p-3 flex items-center justify-between border border-gray-100">
                        <button
                            on:click={
                                let id = id_qty.clone();
                                let qty = item.quantity;
                                move |_| on_update_qty.run((id.clone(), (qty - 0.5).max(0.0)))
                            }
                            class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-white text-gray-400 hover:text-black transition-all font-bold text-lg active:scale-90"
                        >
                            "-"
                        </button>

                        <div class="flex flex-col items-center">
                            <span class="text-xl font-bold text-gray-900 tracking-tight leading-none">{item.quantity}</span>
                            <span class="text-[9px] font-bold text-gray-400 uppercase tracking-widest mt-1">{item.unit.clone()}</span>
                        </div>

                        <button
                            on:click={
                                let id = id_qty.clone();
                                let qty = item.quantity;
                                move |_| on_update_qty.run((id.clone(), qty + 0.5))
                            }
                            class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-white text-gray-400 hover:text-black transition-all font-bold text-lg active:scale-90"
                        >
                            "+"
                        </button>
                    </div>
                </div>
            </div>

            <div class=format!("text-center py-2 px-3 rounded-lg text-[9px] font-bold tracking-widest border transition-all {}", status_class)>
                {status_text}
            </div>
        </div>
    }
}
