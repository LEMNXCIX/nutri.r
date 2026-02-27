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
                    ("EXPIRED", "text-red-500 bg-red-50 border-red-100")
                } else if days_until <= 7 {
                    (
                        "CRITICAL STOCK",
                        "text-[#D4AF37] bg-black border-black shadow-lg shadow-black/20",
                    )
                } else {
                    ("STABLE STOCK", "text-gray-400 bg-white border-gray-100")
                }
            } else {
                ("INVALID DATE", "text-gray-300 bg-gray-50 border-gray-100")
            }
        } else {
            ("PERPETUAL", "text-gray-400 bg-white border-gray-100")
        }
    };

    let (status_text, status_class) = match expiration_status() {
        (text, "text-red-500 bg-red-50 border-red-100") => (text, "bg-red-500 text-white"),
        (text, "text-[#D4AF37] bg-black border-black shadow-lg shadow-black/20") => {
            (text, "bg-accent text-neutral-950")
        }
        (text, _) => (text, "bg-neutral-950 text-white"),
    };

    let id_del = item.id.clone();
    let id_qty = item.id.clone();

    view! {
        <div class="bg-white dark:bg-neutral-900 p-8 brutalist-border dark:border-neutral-700 flex flex-col justify-between h-full group relative overflow-hidden transition-all hover:bg-neutral-50 dark:hover:bg-neutral-800 shadow-brutalist">
            // Actions - High Contrast
            <div class="absolute top-4 right-4 flex gap-0.5 opacity-0 group-hover:opacity-100 transition-all z-20">
                <button
                    on:click={
                        let item = item.clone();
                        move |ev| {
                            ev.stop_propagation();
                            on_edit.run(item.clone());
                        }
                    }
                    class="w-10 h-10 brutalist-border dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-950 dark:text-white flex items-center justify-center hover:bg-accent transition-colors"
                >
                    <span class="material-symbols-outlined !text-[18px]">"edit"</span>
                </button>
                <button
                    on:click={
                        let id = id_del.clone();
                        move |ev| {
                            ev.stop_propagation();
                            on_delete.run(id.clone());
                        }
                    }
                    class="w-10 h-10 brutalist-border dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-950 dark:text-white flex items-center justify-center hover:bg-red-500 hover:text-white transition-colors"
                >
                    <span class="material-symbols-outlined !text-[18px]">"delete"</span>
                </button>
            </div>

            <div class="space-y-6 relative z-10">
                <div class="space-y-1">
                    <span class="text-[9px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-[0.4em]">{item.category.clone()}</span>
                    <h3 class="text-xl font-black text-neutral-950 dark:text-white tracking-tighter leading-none pr-12 group-hover:text-accent transition-colors uppercase">{item.name.clone()}</h3>
                </div>

                // Quantity Controls
                <div class="brutalist-border dark:border-neutral-700 bg-white dark:bg-neutral-800 p-1 flex items-center justify-between">
                    <button
                        on:click={
                            let id = id_qty.clone();
                            let qty = item.quantity;
                            move |_| on_update_qty.run((id.clone(), (qty - 0.5).max(0.0)))
                        }
                        class="w-12 h-12 flex items-center justify-center bg-white dark:bg-neutral-800 text-neutral-400 dark:text-neutral-500 hover:text-neutral-950 dark:hover:text-white transition-colors font-black text-xl"
                    >
                        <span class="material-symbols-outlined">"remove"</span>
                    </button>

                    <div class="flex flex-col items-center">
                        <span class="text-3xl font-black text-neutral-950 dark:text-white tracking-tighter tabular-nums leading-none">{item.quantity}</span>
                        <span class="text-[8px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-widest mt-1">{item.unit.clone()}</span>
                    </div>

                    <button
                        on:click={
                            let id = id_qty.clone();
                            let qty = item.quantity;
                            move |_| on_update_qty.run((id.clone(), qty + 0.5))
                        }
                        class="w-12 h-12 flex items-center justify-center bg-accent text-neutral-950 transition-colors"
                    >
                        <span class="material-symbols-outlined">"add"</span>
                    </button>
                </div>
            </div>

            <div class=format!("mt-8 py-2 px-4 text-[9px] font-black tracking-[0.3em] transition-all uppercase text-center brutalist-border {}", status_class)>
                {status_text}
            </div>
        </div>
    }
}
