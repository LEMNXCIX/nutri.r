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
                    ("CADUCADO", "text-red-500 bg-red-50 border-red-100")
                } else if days_until <= 7 {
                    (
                        "DISPONIBILIDAD CRÍTICA",
                        "text-[#D4AF37] bg-black border-black shadow-lg shadow-black/20",
                    )
                } else {
                    ("STOCK ESTABLE", "text-gray-400 bg-white border-gray-100")
                }
            } else {
                ("FECHA INVÁLIDA", "text-gray-300 bg-gray-50 border-gray-100")
            }
        } else {
            ("PERMANENTE", "text-gray-400 bg-white border-gray-100")
        }
    };

    let (dot_class, text_class, status_label) = match expiration_status() {
        (text, "text-red-500 bg-red-50 border-red-100") => ("bg-error", "text-error", "Caducado"),
        (text, "text-[#D4AF37] bg-black border-black shadow-lg shadow-black/20") => {
            ("bg-accent", "text-accent", "Disponibilidad Crítica")
        }
        _ => ("bg-neutral-200", "text-neutral-400", "Stock Normal"),
    };

    let id_del = item.id.clone();
    let id_qty = item.id.clone();

    view! {
        <div class="py-10 border-b border-neutral-100 dark:border-neutral-800">
            <div class="flex justify-between items-start mb-6">
                <div>
                    <h2 class="text-3xl font-extrabold uppercase tracking-tight mb-1 text-neutral-950 dark:text-white">{item.name.clone()}</h2>
                    <div class="flex items-center gap-2">
                        <div class=format!("w-2 h-2 rounded-full {}", dot_class)></div>
                        <span class=format!("text-[10px] font-bold uppercase tracking-widest {}", text_class)>{status_label}</span>
                    </div>
                </div>
                <div class="flex gap-4">
                    <button
                        on:click={
                            let item = item.clone();
                            move |ev| {
                                ev.stop_propagation();
                                on_edit.run(item.clone());
                            }
                        }
                        class="text-neutral-300 hover:text-neutral-950 dark:hover:text-white transition-colors"
                    >
                        <span class="material-symbols-outlined !text-[20px]">"edit"</span>
                    </button>
                    <button
                        on:click={
                            let id = id_del.clone();
                            move |ev| {
                                ev.stop_propagation();
                                on_delete.run(id.clone());
                            }
                        }
                        class="text-neutral-300 hover:text-error transition-colors"
                    >
                        <span class="material-symbols-outlined !text-[20px]">"delete"</span>
                    </button>
                </div>
            </div>

            <div class="flex items-center justify-between">
                <div class="flex items-baseline gap-1">
                    <span class="text-5xl font-light tracking-tighter text-neutral-950 dark:text-white">{item.quantity}</span>
                    <span class="text-sm font-bold text-neutral-400 uppercase">{item.unit.clone()}</span>
                </div>
                <div class="flex items-center gap-4 md:gap-6">
                    <button
                        on:click={
                            let id = id_qty.clone();
                            move |_| on_update_qty.run((id.clone(), 0.0))
                        }
                        class="w-10 h-10 flex items-center justify-center rounded-full border border-neutral-100 dark:border-neutral-800 text-neutral-400 hover:border-error dark:hover:border-error hover:text-error dark:hover:text-error transition-all"
                        title="Limpiar stock (marcar como agotado)"
                    >
                        <span class="material-symbols-outlined !text-[20px]">"layers_clear"</span>
                    </button>
                    <button
                        on:click={
                            let id = id_qty.clone();
                            let qty = item.quantity;
                            move |_| on_update_qty.run((id.clone(), (qty - 0.5).max(0.0)))
                        }
                        class="w-10 h-10 flex items-center justify-center rounded-full border border-neutral-100 dark:border-neutral-800 text-neutral-400 hover:border-neutral-950 dark:hover:border-white hover:text-neutral-950 dark:hover:text-white transition-all"
                    >
                        <span class="material-symbols-outlined !text-[20px]">"remove"</span>
                    </button>
                    <button
                        on:click={
                            let id = id_qty.clone();
                            let qty = item.quantity;
                            move |_| on_update_qty.run((id.clone(), qty + 0.5))
                        }
                        class="w-10 h-10 flex items-center justify-center rounded-full border border-neutral-100 dark:border-neutral-800 text-neutral-400 hover:border-neutral-950 dark:hover:border-white hover:text-neutral-950 dark:hover:text-white transition-all"
                    >
                        <span class="material-symbols-outlined !text-[20px]">"add"</span>
                    </button>
                </div>
            </div>
        </div>
    }
}
