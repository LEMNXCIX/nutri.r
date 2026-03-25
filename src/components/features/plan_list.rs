use crate::plan_display::{format_plan_created_at, plan_display_name};
use crate::tauri_bridge::PlanIndex;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn PlanList(#[prop(into)] plans: Signal<Vec<PlanIndex>>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 pb-20">
            {move || plans.get().into_iter().map(|plan| view! {
                <PlanItem plan=plan />
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn PlanItem(plan: PlanIndex) -> impl IntoView {
    let title = plan_display_name(&plan);
    let created_at = format_plan_created_at(&plan);

    view! {
        <div class="group relative overflow-hidden rounded-[2.5rem] bg-white dark:bg-neutral-900 border border-gray-100 dark:border-neutral-800 shadow-xl shadow-black/5 hover:shadow-2xl hover:border-black/5 dark:hover:border-neutral-700 transition-all duration-500 animate-in fade-in slide-in-from-bottom-4">
            // Favorite Badge - Refined
            {if plan.is_favorite {
                view! {
                    <div class="absolute top-6 right-6 z-20">
                        <div class="p-2.5 rounded-xl bg-black dark:bg-neutral-800 border border-black dark:border-neutral-700 shadow-lg shadow-black/20">
                            <svg class="w-4 h-4 text-[#D4AF37] fill-current" viewBox="0 0 24 24">
                                <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
                            </svg>
                        </div>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}

            <A href={format!("/plan/{}", plan.id)} attr:class="block p-8 space-y-6">
                // Card Header
                <div class="space-y-2">
                    <div class="flex items-center gap-2">
                        <span class="h-px w-4 bg-[#D4AF37]"></span>
                        <span class="text-[9px] font-black text-[#D4AF37] uppercase tracking-[0.2em]">
                            {if plan.enviado { "PUBLISHED" } else { "DRAFT" }}
                        </span>
                    </div>
                    <h3 class="text-2xl font-black text-black dark:text-white tracking-tighter leading-none group-hover:text-[#D4AF37] transition-colors">
                        {title}
                    </h3>
                    <p class="text-[10px] font-bold uppercase tracking-[0.2em] text-gray-400 dark:text-neutral-500">
                        {created_at}
                    </p>
                </div>

                // Rating & ID
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-1.5">
                         {if let Some(r) = plan.rating {
                            view! {
                                <div class="flex gap-0.5">
                                    {(1..=5).map(|i| {
                                        let color = if i <= r { "text-black dark:text-white fill-current" } else { "text-gray-100 dark:text-neutral-700 fill-gray-50 dark:fill-neutral-800" };
                                        view! { <svg class=format!("w-3 h-3 {}", color) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg> }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        } else {
                            view! { <span class="text-[8px] text-gray-300 dark:text-neutral-600 font-black uppercase tracking-widest">"PENDING RATE"</span> }.into_any()
                        }}
                    </div>
                    <span class="text-[8px] font-black text-gray-400 dark:text-neutral-500 bg-[#FAFAFA] dark:bg-neutral-800 border border-gray-100 dark:border-neutral-700 px-3 py-1 rounded-lg uppercase tracking-widest">
                        {format!("REF: {}", plan.id.chars().take(6).collect::<String>())}
                    </span>
                </div>

                // Content Preview (Proteins) - Clean Bubbles
                <div class="flex flex-wrap gap-2">
                    {plan.proteinas.iter().take(3).map(|p| {
                        view! {
                            <div class="px-3 py-1.5 rounded-xl bg-gray-50 dark:bg-neutral-800 border border-transparent dark:border-neutral-700 text-[9px] font-black text-gray-500 dark:text-neutral-400 uppercase tracking-widest group-hover:bg-black group-hover:text-[#D4AF37] transition-all duration-500">
                                {p.clone()}
                            </div>
                        }
                    }).collect_view()}
                </div>

                // Action Footer
                <div class="pt-6 border-t border-gray-50 dark:border-neutral-800 flex items-center justify-between">
                    <div class="flex items-center gap-3 text-[9px] font-black text-gray-400 dark:text-neutral-500 uppercase tracking-[0.2em] group-hover:text-black dark:group-hover:text-white transition-colors">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                        </svg>
                        "View Insights"
                    </div>
                    <div class="w-10 h-10 rounded-2xl bg-gray-50 dark:bg-neutral-800 flex items-center justify-center text-gray-400 dark:text-neutral-500 group-hover:bg-black group-hover:text-[#D4AF37] transition-all duration-500">
                        <svg class="w-5 h-5 group-hover:translate-x-0.5 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M9 5l7 7-7 7" />
                        </svg>
                    </div>
                </div>
            </A>
        </div>
    }
}
