use crate::components::features::PlanList;
use crate::components::ui::Loading;
use crate::tauri_bridge::get_index;
use leptos::prelude::*;

#[component]
pub fn History() -> impl IntoView {
    let plans_resource = LocalResource::new(move || async move { get_index().await });

    view! {
        <div class="p-4 md:p-6 max-w-5xl mx-auto animate-in fade-in duration-500">
            <header class="mb-8">
                <div class="flex items-center gap-3 mb-2">
                    <svg class="w-8 h-8 text-black fill-none stroke-current stroke-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round">
                         <path d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <h2 class="text-3xl font-black text-gray-900 tracking-tight">"Historial de Planes"</h2>
                </div>
                <p class="text-gray-500 font-medium">"Todos tus planes generados, ordenados cronológicamente."</p>
            </header>

            <Suspense fallback=move || view! { <div class="flex justify-center p-12"><Loading /></div> }>
                {move || {
                    match plans_resource.get() {
                        Some(Ok(plans)) => {
                            if plans.is_empty() {
                                view! {
                                    <div class="text-center p-16 bg-gray-50 rounded-3xl border border-dashed border-gray-200">
                                        <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-white border border-gray-100 mb-4 shadow-sm">
                                            <svg class="w-8 h-8 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                                            </svg>
                                        </div>
                                        <h3 class="text-xl font-bold text-gray-900 mb-2">"Aún no tienes planes"</h3>
                                        <p class="text-gray-500 max-w-xs mx-auto text-sm">"Genera un nuevo plan semanal desde el Dashboard para comenzar."</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <PlanList plans=Signal::derive(move || plans.clone()) />
                                }.into_any()
                            }
                        }
                        Some(Err(e)) => {
                            view! {
                                <div class="p-6 bg-red-50 border border-red-100 rounded-2xl text-red-600 text-sm font-medium">
                                    {format!("Error al cargar el historial: {}", e)}
                                </div>
                            }.into_any()
                        }
                        None => view! { <div class="flex justify-center p-12"><Loading /></div> }.into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
