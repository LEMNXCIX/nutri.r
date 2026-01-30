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
                    (
                        "VENCIDO",
                        "text-red-400 bg-red-500/10 border-red-500/20 shadow-[0_0_15px_rgba(239,68,68,0.1)]",
                    )
                } else if days_until <= 7 {
                    (
                        "VENCE PRONTO",
                        "text-orange-400 bg-orange-400/10 border-orange-400/20 shadow-[0_0_15px_rgba(251,146,60,0.1)]",
                    )
                } else {
                    (
                        "EN ESTADO",
                        "text-green-400 bg-green-500/10 border-green-500/20 shadow-[0_0_15px_rgba(34,197,94,0.1)]",
                    )
                }
            } else {
                ("FECHA INVÁLIDA", "text-gray-500 bg-gray-900 border-white/5")
            }
        } else {
            (
                "SIN VENCIMIENTO",
                "text-gray-500 bg-gray-900 border-white/5",
            )
        }
    };

    let (status_text, status_class) = expiration_status();
    let id_del = item.id.clone();
    let id_qty = item.id.clone();

    view! {
        <div class="glass-light p-6 rounded-[2.5rem] border-white/5 hover:bg-white/5 transition-all duration-500 group relative overflow-hidden flex flex-col justify-between h-full">
            <div class="absolute -top-12 -right-12 w-32 h-32 bg-blue-500/5 blur-3xl group-hover:bg-blue-500/10 transition-all duration-700"></div>

            <div>
                <div class="flex justify-between items-start mb-4">
                    <div class="flex-1">
                        <span class="text-[9px] font-black uppercase tracking-[0.2em] text-blue-500/80 mb-1 block">{item.category.clone()}</span>
                        <h3 class="text-lg font-black text-white tracking-tighter uppercase italic group-hover:text-blue-400 transition-colors">{item.name.clone()}</h3>
                    </div>

                    <div class="flex gap-2">
                        <button
                            type="button"
                            on:click={
                                let item = item.clone();
                                move |ev| {
                                    ev.stop_propagation();
                                    on_edit.run(item.clone());
                                }
                            }
                            class="p-3 rounded-2xl bg-white/5 text-blue-400 hover:text-blue-300 hover:bg-blue-500/20 transition-all active:scale-95 flex items-center justify-center z-10 border border-white/5 shadow-xl"
                            title="Editar producto"
                        >
                            <svg class="w-4 h-4 pointer-events-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
                            class="p-3 rounded-2xl bg-white/5 text-red-400 hover:text-red-300 hover:bg-red-500/20 transition-all active:scale-95 flex items-center justify-center z-10 border border-white/5 shadow-xl"
                            title="Eliminar producto"
                        >
                            <svg class="w-4 h-4 pointer-events-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="flex items-center gap-4 mb-6">
                    <div class="flex-1 glass-light rounded-2xl p-4 flex items-center justify-between border-white/5 shadow-inner">
                        <button
                            on:click={
                                let id = id_qty.clone();
                                let qty = item.quantity;
                                move |_| on_update_qty.run((id.clone(), (qty - 0.5).max(0.0)))
                            }
                            class="w-8 h-8 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 text-white transition-all font-black text-lg active:scale-90"
                        >
                            "-"
                        </button>

                        <div class="flex flex-col items-center">
                            <span class="text-2xl font-black text-white tracking-tighter italic leading-none">{item.quantity}</span>
                            <span class="text-[8px] font-black text-gray-500 uppercase tracking-widest mt-1">{item.unit.clone()}</span>
                        </div>

                        <button
                            on:click={
                                let id = id_qty.clone();
                                let qty = item.quantity;
                                move |_| on_update_qty.run((id.clone(), qty + 0.5))
                            }
                            class="w-8 h-8 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 text-white transition-all font-black text-lg active:scale-90"
                        >
                            "+"
                        </button>
                    </div>
                </div>
            </div>

            <div class=format!("text-center py-2 px-4 rounded-xl text-[9px] font-black tracking-[0.2em] border transition-all {}", status_class)>
                {status_text}
            </div>
        </div>
    }
}
