use leptos::prelude::*;

#[component]
pub fn WaterTracker(
    current: Signal<f32>,
    target: f32,
    on_add: Callback<()>,
    on_remove: Callback<()>,
) -> impl IntoView {
    let total_droplets = 10;

    // Calculate filled droplets based on percentage
    // specific logic: if target is 2.5L, and current is 1.25L, 50% -> 5 droplets
    let filled_droplets = move || {
        let pct = (current.get() / target).clamp(0.0, 1.0);
        (pct * total_droplets as f32).round() as i32
    };

    view! {
        <div class="card p-6 flex flex-col justify-between bg-blue-50/50 hover:bg-blue-50 transition-colors border-blue-100">
            <div class="flex justify-between items-start mb-6">
                <div>
                    <h3 class="font-bold text-gray-900 text-lg">"Hidratación"</h3>
                    <p class="text-xs text-gray-500 font-medium mt-1">"Meta Diaria: " {target} "L"</p>
                </div>
                <div class="w-10 h-10 rounded-xl bg-blue-100 flex items-center justify-center text-blue-600">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12a8 8 0 00-16 0c0-4.418 3.582-8 8-8s8 3.582 8 8z" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 17a4 4 0 100-8 4 4 0 000 8z" />
                    </svg>
                </div>
            </div>

            <div class="flex items-end justify-between mb-4">
                <div>
                    <span class="text-3xl font-bold text-gray-900">{move || format!("{:.1}", current.get())}</span>
                    <span class="text-sm text-gray-400 font-medium ml-1">"L"</span>
                </div>
                <div class="flex gap-2">
                    <button
                        on:click=move |_| on_remove.run(())
                        class="w-8 h-8 rounded-lg bg-white border border-blue-100 text-blue-500 hover:bg-blue-50 flex items-center justify-center transition-all active:scale-95"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" /></svg>
                    </button>
                    <button
                        on:click=move |_| on_add.run(())
                        class="w-8 h-8 rounded-lg bg-blue-500 text-white shadow-soft-blue flex items-center justify-center hover:bg-blue-600 transition-all active:scale-95"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" /></svg>
                    </button>
                </div>
            </div>

            <div class="flex items-center gap-1.5 h-8">
                {(0..total_droplets).map(|i| {
                    view! {
                        <div class="flex-1 h-full flex items-end group relative">
                            <div class=move || format!(
                                "w-full rounded-full transition-all duration-500 {}",
                                if i < filled_droplets() { "bg-blue-400 h-full" } else { "bg-blue-100 h-2" }
                            )></div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
