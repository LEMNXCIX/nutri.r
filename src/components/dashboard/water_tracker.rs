use leptos::prelude::*;

#[component]
pub fn WaterTracker(
    current: Signal<f32>,
    target: f32,
    on_add: Callback<()>,
    on_remove: Callback<()>,
) -> impl IntoView {
    let total_bars = 12;

    let filled_bars = move || {
        let pct = (current.get() / target).clamp(0.0, 1.0);
        (pct * total_bars as f32).round() as i32
    };

    view! {
        <div class="bg-white dark:bg-neutral-900 p-6 brutalist-border dark:border-neutral-700 flex flex-col justify-between group relative overflow-hidden">
            <header class="flex justify-between items-start mb-8 relative z-10">
                <div class="space-y-1">
                    <span class="text-[9px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-[0.4em]">"Hydration"</span>
                    <h3 class="font-black text-neutral-950 dark:text-white text-xl tracking-tighter uppercase">"Water Logger"</h3>
                </div>
                <div class="flex items-center gap-2 bg-neutral-950 dark:bg-neutral-700 px-2 py-1">
                    <span class="text-[10px] font-black text-accent uppercase tracking-widest">{target} "L"</span>
                </div>
            </header>

            <div class="flex items-center justify-between mb-8 relative z-10">
                <div class="flex items-baseline gap-1">
                    <span class="text-5xl font-black text-neutral-950 dark:text-white tracking-tighter tabular-nums">{move || format!("{:.1}", current.get())}</span>
                    <span class="text-[10px] font-bold text-neutral-400 dark:text-neutral-500 uppercase tracking-widest">"LT"</span>
                </div>
                <div class="flex gap-2">
                    <button
                        on:click=move |_| on_remove.run(())
                        class="w-10 h-10 brutalist-border dark:border-neutral-600 bg-white dark:bg-neutral-800 text-neutral-950 dark:text-white hover:bg-neutral-50 dark:hover:bg-neutral-700 flex items-center justify-center transition-transform active:scale-95"
                    >
                        <span class="material-symbols-outlined !text-[20px]">"remove"</span>
                    </button>
                    <button
                        on:click=move |_| on_add.run(())
                        class="w-10 h-10 brutalist-border bg-accent text-neutral-950 flex items-center justify-center transition-transform active:scale-95"
                    >
                        <span class="material-symbols-outlined !text-[20px]">"add"</span>
                    </button>
                </div>
            </div>

            // Technical Progress Bar
            <div class="flex items-center gap-1 h-2 relative z-10">
                {(0..total_bars).map(|i| {
                    view! {
                        <div class="flex-1 h-full relative">
                            <div class=move || format!(
                                "absolute inset-0 transition-all duration-500 {}",
                                if i < filled_bars() { "bg-accent" } else { "bg-neutral-100 dark:bg-neutral-700" }
                            )></div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
