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
    let radius = 58;
    let circumference = 2.0 * std::f32::consts::PI * radius as f32;
    let stroke_dashoffset = circumference - (percentage / 100.0) * circumference;

    view! {
        <div class="card p-6 flex flex-col justify-center">
            <div class="flex flex-col xl:flex-row items-center gap-6 xl:gap-4">
                // Circle Chart
                <div class="relative w-32 h-32 shrink-0">
                    <svg class="w-full h-full transform -rotate-90">
                        // Background Circle
                        <circle
                            cx="64" cy="64" r="58"
                            stroke="#f4f4f5" // gray-100
                            stroke-width="8"
                            fill="none"
                        />
                        // Progress Circle
                        <circle
                            cx="64" cy="64" r="58"
                            stroke="#a1a1aa" // gray-400
                            stroke-width="8"
                            fill="none"
                            stroke-dasharray={circumference.to_string()}
                            stroke-dashoffset={stroke_dashoffset.to_string()}
                            stroke-linecap="round"
                            class="transition-all duration-1000 ease-out"
                        />
                    </svg>

                    // Center Text
                    <div class="absolute inset-0 flex flex-col items-center justify-center text-center">
                        <span class="text-2xl font-bold text-gray-900">{calories_current}</span>
                        <span class="text-[10px] text-gray-400 font-medium mt-0.5">{calories_target} " kcal"</span>
                    </div>
                </div>

                // Macros
                <div class="flex-1 w-full space-y-3">
                    <MacroRow label="Proteína" percentage=protein_pct icon="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                    <MacroRow label="Carbohidratos" percentage=carbs_pct icon="M12 2a10 10 0 0 1 10 10 10 10 0 0 1-5 8.66V6H7v14.66A10 10 0 0 1 2 12 10 10 0 0 1 12 2z" />
                    <MacroRow label="Grasas" percentage=fat_pct icon="M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18z" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn MacroRow(label: &'static str, percentage: i32, icon: &'static str) -> impl IntoView {
    view! {
        <div class="space-y-1.5">
            <div class="flex items-center justify-between text-sm">
                <div class="flex items-center gap-2 text-gray-700 font-medium">
                    <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon />
                    </svg>
                    {label}
                </div>
                <span class="text-gray-400 font-medium">{percentage} "%"</span>
            </div>
            <div class="h-2 w-full bg-gray-100 rounded-full overflow-hidden">
                <div
                    class="h-full bg-gray-500 rounded-full transition-all duration-1000 ease-out"
                    style=format!("width: {}%", percentage)
                ></div>
            </div>
        </div>
    }
}
