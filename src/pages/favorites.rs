use crate::components::features::PlanList;
use crate::tauri_bridge::get_favorite_plans;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Favorites() -> impl IntoView {
    let plans_resource = LocalResource::new(move || async move { get_favorite_plans().await });

    view! {
        <div class="bg-[#FAFAFA] min-h-screen font-sans text-[#171717] pb-32 selection:bg-[#D4AF37]/30">
            // Header - Editorial Style
            <header class="bg-white border-b border-gray-100 pb-20 pt-16 px-4 shadow-sm relative overflow-hidden mb-12">
                <div class="absolute top-0 right-0 w-64 h-64 bg-[#D4AF37]/5 -mr-32 -mt-32 rounded-full blur-3xl"></div>
                
                <div class="max-w-5xl mx-auto relative z-10">
                    <div class="space-y-6">
                         <div class="space-y-2">
                            <div class="flex items-center gap-3">
                                <span class="h-px w-8 bg-[#D4AF37]"></span>
                                <span class="text-[10px] font-black text-[#D4AF37] tracking-[0.3em] uppercase">"Colección Privada"</span>
                            </div>
                            <h2 class="text-5xl md:text-6xl font-black text-black tracking-tighter leading-none">
                                "MIS FAVORITOS"
                            </h2>
                            <p class="text-gray-500 font-medium max-w-lg mt-4 text-sm leading-relaxed">
                                "Una selección curada de tus mejores planes nutricionales. Accede rápidamente a las dietas que mejor se adaptan a tu estilo de vida."
                            </p>
                         </div>
                    </div>
                </div>
            </header>

            <div class="max-w-5xl mx-auto px-4">
                <Suspense fallback=move || view! { 
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div class="h-64 bg-white rounded-[2.5rem] animate-pulse"></div>
                        <div class="h-64 bg-white rounded-[2.5rem] animate-pulse"></div>
                        <div class="h-64 bg-white rounded-[2.5rem] animate-pulse"></div>
                    </div>
                }>
                    {move || {
                        let content: AnyView = match plans_resource.get() {
                            Some(Ok(plans)) => {
                                if plans.is_empty() {
                                    view! {
                                        <div class="text-center py-32 px-8 bg-white rounded-[3rem] border border-gray-100 shadow-xl shadow-black/5 max-w-2xl mx-auto">
                                            <div class="w-24 h-24 bg-gray-50 rounded-full flex items-center justify-center mx-auto mb-8 text-gray-300">
                                                <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                                                </svg>
                                            </div>
                                            <h3 class="text-2xl font-black text-black tracking-tighter mb-4">"TU GALERÍA ESTÁ VACÍA"</h3>
                                            <p class="text-gray-500 text-sm max-w-xs mx-auto mb-10 leading-relaxed font-medium">"Explora nuevos planes y marca con una estrella los que desees guardar en tu colección personal."</p>
                                            <A href="/" attr:class="inline-block bg-black hover:bg-[#D4AF37] text-white px-10 py-5 rounded-[1.5rem] font-black text-[11px] tracking-[0.2em] uppercase shadow-2xl shadow-black/20 transition-all">
                                                "EXPLORAR PLANES"
                                            </A>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="animate-in fade-in slide-in-from-bottom-4 duration-700">
                                            <PlanList plans=Signal::derive(move || plans.clone()) />
                                        </div>
                                    }.into_any()
                                }
                            }
                            Some(Err(e)) => {
                                view! {
                                    <div class="p-8 bg-red-50 border border-red-100 rounded-[2rem] text-red-600 text-sm font-bold flex items-center gap-4">
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                        {format!("ERROR AL CARGAR FAVORITOS: {}", e)}
                                    </div>
                                }.into_any()
                            }
                            None => view! { <div /> }.into_any()
                        };
                        content
                    }}
                </Suspense>
            </div>
        </div>
    }
}
