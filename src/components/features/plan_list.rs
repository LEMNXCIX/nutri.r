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
    view! {
        <div class="group relative overflow-hidden rounded-[2rem] bg-white border border-gray-200 shadow-sm hover:shadow-xl transition-all duration-300 animate-in fade-in slide-in-from-bottom-4">
            // Favorite Badge
            {if plan.is_favorite {
                view! {
                    <div class="absolute top-5 right-5 z-20">
                        <div class="p-2 rounded-xl bg-white/80 backdrop-blur-md border border-red-100 shadow-sm">
                            <svg class="w-4 h-4 text-red-500 fill-current" viewBox="0 0 24 24">
                                <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
                            </svg>
                        </div>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}

            <A href={format!("/plan/{}", plan.id)} attr:class="block space-y-5 p-6">
                // Card Header
                <div class="space-y-1">
                    <span class="text-[10px] font-black text-gray-400 uppercase tracking-[0.2em]">
                        {if plan.enviado { "Publicado" } else { "Borrador" }}
                    </span>
                    <h3 class="text-xl font-black text-gray-900 tracking-tighter group-hover:text-black transition-colors">
                        {plan.fecha}
                    </h3>
                </div>

                // Rating & ID
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-1">
                         {if let Some(r) = plan.rating {
                            view! {
                                <div class="flex gap-0.5">
                                    {(1..=5).map(|i| {
                                        let color = if i <= r { "text-black fill-current" } else { "text-gray-200 fill-gray-100" };
                                        view! { <svg class=format!("w-3.5 h-3.5 {}", color) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg> }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        } else {
                            view! { <span class="text-[10px] text-gray-300 font-bold uppercase tracking-wider">"Sin valorar"</span> }.into_any()
                        }}
                    </div>
                    <span class="text-[9px] font-mono text-gray-500 bg-gray-50 border border-gray-100 px-2 py-0.5 rounded-md">
                        {plan.id.chars().take(8).collect::<String>()}
                    </span>
                </div>

                // Content Preview (Proteins)
                <div class="space-y-3">
                    <div class="flex flex-wrap gap-2">
                        {plan.proteinas.iter().take(3).map(|p| {
                            view! {
                                <div class="px-3 py-1.5 rounded-xl bg-gray-50 border border-gray-100 text-[9px] font-black text-gray-500 uppercase tracking-widest group-hover:bg-gray-100 group-hover:text-black transition-all">
                                    {p.clone()}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Action Footer
                <div class="pt-4 border-t border-gray-100 flex items-center justify-between">
                    <div class="flex items-center gap-2 text-[10px] text-gray-400 font-black uppercase tracking-tighter group-hover:text-black transition-colors">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                        </svg>
                        "Ver Detalle"
                    </div>
                    <div class="w-8 h-8 rounded-full bg-gray-50 border border-gray-100 flex items-center justify-center text-gray-400 group-hover:bg-black group-hover:text-white group-hover:border-black transition-all">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                    </div>
                </div>
            </A>
        </div>
    }
}
