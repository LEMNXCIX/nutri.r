use leptos::prelude::*;

#[component]
pub fn NutritionSummary(
    #[prop(into)] calories_current: i32,
    #[prop(into)] calories_target: i32,
    #[prop(into)] protein_pct: i32,
    #[prop(into)] carbs_pct: i32,
    #[prop(into)] fat_pct: i32,
) -> impl IntoView {
    let percentage = (calories_current as f32 / calories_target as f32) * 100.0;

    // Circle math
    let radius = 54;
    let circumference = 2.0 * std::f32::consts::PI * radius as f32;
    let stroke_dashoffset = circumference - (percentage.min(100.0) / 100.0) * circumference;

    view! {
        <div class="bg-white dark:bg-neutral-900 p-6 brutalist-border dark:border-neutral-700 flex flex-col justify-center relative overflow-hidden group">
            <div class="flex flex-col xl:flex-row items-center gap-8 relative z-10">
                // Circle Chart - Brutalist & Clean
                <div class="relative w-32 h-32 shrink-0 flex items-center justify-center">
                    <svg class="w-full h-full transform -rotate-90">
                        <circle
                            cx="64" cy="64" r="58"
                            stroke="currentColor"
                            class="text-gray-100 dark:text-neutral-700"
                            stroke-width="1"
                            fill="none"
                        />
                        <circle
                            cx="64" cy="64" r="58"
                            stroke="#0df259"
                            stroke-width="8"
                            fill="none"
                            stroke-dasharray={circumference.to_string()}
                            stroke-dashoffset={stroke_dashoffset.to_string()}
                            class="transition-all duration-1000 ease-out"
                        />
                    </svg>

                    // Center Text
                    <div class="absolute inset-0 flex flex-col items-center justify-center text-center">
                        <span class="text-3xl font-black text-neutral-950 dark:text-white tracking-tighter leading-none">{calories_current}</span>
                        <div class="hairline-divider dark:bg-neutral-700 w-8 my-1"></div>
                        <span class="text-[8px] text-neutral-400 dark:text-neutral-500 font-bold uppercase tracking-[0.2em]">{calories_target}</span>
                    </div>
                </div>

                // Macros - High Contrast High Tracked
                <div class="flex-1 w-full space-y-4">
                    <h4 class="text-[9px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-[0.4em]">"Nutrition Analysis"</h4>
                    <div class="grid grid-cols-1 gap-4">
                        <MacroRow label="Protein" percentage=protein_pct color="#000" />
                        <MacroRow label="Carbohydrates" percentage=carbs_pct color="#0df259" />
                        <MacroRow label="Fats" percentage=fat_pct color="#666" />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MacroRow(label: &'static str, percentage: i32, color: &'static str) -> impl IntoView {
    view! {
        <div class="space-y-1.5">
            <div class="flex items-center justify-between">
                <span class="text-[8px] font-bold text-neutral-500 dark:text-neutral-400 uppercase tracking-[0.2em]">{label}</span>
                <span class="text-[9px] font-black text-neutral-950 dark:text-white tabular-nums">{percentage} "%"</span>
            </div>
            <div class="h-[1px] w-full bg-neutral-100 dark:bg-neutral-700 relative">
                <div
                    class="h-full absolute top-0 left-0 transition-all duration-1000 ease-out"
                    style=format!("width: {}%; background-color: {};", percentage.min(100), color)
                ></div>
            </div>
        </div>
    }
}
