use leptos::prelude::*;

#[component]
pub fn MealCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] calories: i32,
    #[prop(into)] icon: String,
) -> impl IntoView {
    view! {
        <div class="bg-white p-4 brutalist-border hover:bg-neutral-50 transition-colors group cursor-pointer flex items-center gap-4 relative overflow-hidden">
            // Icon - High Contrast Square
            <div class="w-12 h-12 brutalist-border bg-neutral-950 flex items-center justify-center shrink-0 group-hover:bg-accent transition-colors relative z-10">
                <svg class="w-5 h-5 text-accent group-hover:text-black transition-colors" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon />
                </svg>
            </div>

            // Content - Brutalist Tracked
            <div class="flex-1 min-w-0 relative z-10">
                <div class="flex items-center gap-2 mb-0.5">
                    <h3 class="font-black text-neutral-950 text-xs uppercase tracking-tight">{title}</h3>
                    <div class="hairline-divider w-2"></div>
                    <span class="text-[8px] font-black text-neutral-400 uppercase tracking-widest tabular-nums">{calories} " KCAL"</span>
                </div>
                <p class="text-[9px] text-neutral-500 font-bold uppercase truncate tracking-wider">{description}</p>
            </div>

            // Action
            <div class="text-neutral-300 group-hover:text-neutral-950 transition-all group-hover:translate-x-1 relative z-10">
                <span class="material-symbols-outlined !text-[20px]">"chevron_right"</span>
            </div>
        </div>
    }
}
