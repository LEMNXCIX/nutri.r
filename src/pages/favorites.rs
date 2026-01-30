use crate::components::features::PlanList;
use crate::components::ui::Loading;
use crate::tauri_bridge::get_favorite_plans;
use leptos::prelude::*;

#[component]
pub fn Favorites() -> impl IntoView {
    let plans_resource = LocalResource::new(move || async move { get_favorite_plans().await });

    view! {
        <div class="p-4 md:p-6 max-w-5xl mx-auto">
            <header class="mb-8">
                <div class="flex items-center gap-3 mb-2">
                    <svg class="w-8 h-8 text-red-500 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
                    </svg>
                    <h2 class="text-3xl font-bold text-white">"Mis Planes Favoritos"</h2>
                </div>
                <p class="text-gray-400">"Aquí encontrarás los planes que más te han gustado."</p>
            </header>

            <Suspense fallback=move || view! { <div class="flex justify-center p-12"><Loading /></div> }>
                {move || {
                    match plans_resource.get() {
                        Some(Ok(plans)) => {
                            if plans.is_empty() {
                                view! {
                                    <div class="text-center p-16 bg-gray-800/50 rounded-3xl border border-dashed border-gray-700">
                                        <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-gray-800 mb-4">
                                            <svg class="w-8 h-8 text-gray-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                                            </svg>
                                        </div>
                                        <h3 class="text-xl font-medium text-gray-300 mb-2">"Aún no tienes favoritos"</h3>
                                        <p class="text-gray-500 max-w-xs mx-auto">"Marca planes con el corazón en el detalle del plan para verlos aquí."</p>
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
                                <div class="p-6 bg-red-900/20 border border-red-900/50 rounded-2xl text-red-300">
                                    {format!("Error al cargar favoritos: {}", e)}
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
