use leptos::prelude::*;

#[component]
pub fn MealCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] calories: i32,
    #[prop(into)] icon: String, // SVG path or identifier
) -> impl IntoView {
    view! {
        <div class="card p-4 flex items-center gap-4 group cursor-pointer hover:border-gray-300">
            // Icon Container
            <div class="w-12 h-12 rounded-2xl bg-[#F5F5F0] flex items-center justify-center shrink-0">
                <div class="w-6 h-6 text-gray-700">
                    // Render SVG path passed as prop or use a default
                    <svg viewBox="0 0 24 24" fill="none" class="w-full h-full" stroke="currentColor" stroke-width="1.5">
                        <path stroke-linecap="round" stroke-linejoin="round" d=icon />
                    </svg>
                </div>
            </div>

            // Content
            <div class="flex-1 min-w-0">
                <h3 class="font-bold text-gray-900 text-base">{title}</h3>
                <p class="text-sm text-gray-600 truncate">{description}</p>
                <p class="text-xs font-medium text-gray-400 mt-0.5">{calories} " kcal"</p>
            </div>

            // Action
            <button class="p-2 text-gray-400 hover:text-black rounded-lg hover:bg-gray-50 transition-colors">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                </svg>
            </button>
        </div>
    }
}
