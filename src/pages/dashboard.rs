use crate::components::ui::{Card, Loading};
use crate::tauri_bridge::{get_ingredient_trends, get_statistics};
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    let stats_resource = LocalResource::new(move || async move { get_statistics().await });
    let trends_resource = LocalResource::new(move || async move { get_ingredient_trends().await });

    view! {
        <div class="bg-[#FAFAFA] min-h-screen font-sans text-[#171717] pb-32 selection:bg-[#D4AF37]/30 animate-in fade-in duration-700">
            // Header - Editorial Style
            <header class="bg-white border-b border-gray-100 pb-20 pt-16 px-4 shadow-sm relative overflow-hidden mb-12">
                <div class="absolute top-0 right-0 w-64 h-64 bg-[#D4AF37]/5 -mr-32 -mt-32 rounded-full blur-3xl"></div>
                
                <div class="max-w-5xl mx-auto relative z-10">
                    <div class="space-y-6">
                         <div class="space-y-2">
                            <div class="flex items-center gap-3">
                                <span class="h-px w-8 bg-[#D4AF37]"></span>
                                <span class="text-[10px] font-black text-[#D4AF37] tracking-[0.3em] uppercase">"Data Insights"</span>
                            </div>
                            <h2 class="text-5xl md:text-6xl font-black text-black tracking-tighter leading-none uppercase">
                                "ANALYTICS"
                            </h2>
                            <p class="text-gray-500 font-medium max-w-lg mt-4 text-sm leading-relaxed uppercase tracking-widest text-[10px]">
                                "Monitorea tus tendencias nutricionales y métricas de consumo en tiempo real."
                            </p>
                         </div>
                    </div>
                </div>
            </header>

            <div class="max-w-5xl mx-auto px-4 space-y-12">
                // KPIs Section
                <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                    <Suspense fallback=move || view! { 
                        <div class="contents">
                            {(0..4).map(|_| view! { <div class="h-32 bg-white rounded-[2rem] animate-pulse"></div> }).collect_view()}
                        </div>
                    }>
                        {move || {
                            let content: AnyView = match stats_resource.get() {
                                Some(Ok(s)) => view! {
                                    <MetricCard
                                        label="TOTAL PLANES"
                                        value=s.total_plans.to_string()
                                        icon="📋"
                                    />
                                    <MetricCard
                                        label="FAVORITOS"
                                        value=s.favorite_plans.to_string()
                                        icon="★"
                                        is_gold=true
                                    />
                                    <MetricCard
                                        label="RECETAS"
                                        value=s.recipes_count.to_string()
                                        icon="🍳"
                                    />
                                    <MetricCard
                                        label="INGREDIENTES"
                                        value=s.ingredients_count.to_string()
                                        icon="🌿"
                                    />
                                }.into_any(),
                                _ => view! { <div class="col-span-full py-12 bg-white rounded-[2rem] text-center text-gray-400 text-[10px] font-black uppercase tracking-widest">"Sin estadísticas disponibles"</div> }.into_any()
                            };
                            content
                        }}
                    </Suspense>
                </div>

                // Charts Section
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    // Trends Section
                    <section class="space-y-6">
                        <h3 class="text-[10px] font-black text-gray-400 uppercase tracking-[0.3em] px-2 flex items-center gap-3">
                            <span class="w-1.5 h-1.5 bg-[#D4AF37] rounded-full"></span>
                            "Top Ingredientes"
                        </h3>
                        <Card class="p-8 bg-white rounded-[2.5rem] border border-gray-100 shadow-xl shadow-black/5 min-h-[400px]">
                            <div class="space-y-8">
                                <Suspense fallback=move || view! { <Loading /> }>
                                    {move || {
                                        let content: AnyView = match trends_resource.get() {
                                            Some(Ok(trends)) => {
                                                if trends.is_empty() {
                                                    view! { <div class="text-gray-300 font-black uppercase tracking-widest py-20 text-center text-[10px]">"No hay datos de consumo"</div> }.into_any()
                                                } else {
                                                    let max_count = trends.first().map(|t| t.count).unwrap_or(1) as f32;
                                                    view! {
                                                        <div class="space-y-6">
                                                            {trends.into_iter().take(6).map(|t| {
                                                                let percentage = (t.count as f32 / max_count * 100.0).max(5.0);
                                                                let width = format!("{}%", percentage);
                                                                view! {
                                                                    <div class="group">
                                                                        <div class="flex justify-between items-end mb-2 px-1">
                                                                            <span class="font-black text-[10px] text-black uppercase tracking-widest">{t.name}</span>
                                                                            <span class="text-[10px] font-black text-[#D4AF37]">{t.count}</span>
                                                                        </div>
                                                                        <div class="h-1.5 w-full bg-gray-50 rounded-full overflow-hidden">
                                                                            <div class="h-full bg-black rounded-full transition-all duration-1000 ease-out group-hover:bg-[#D4AF37]" style:width=width></div>
                                                                        </div>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_any()
                                                }
                                            },
                                            _ => view! { <div /> }.into_any()
                                        };
                                        content
                                    }}
                                </Suspense>
                            </div>
                        </Card>
                    </section>

                    // Distribution Section
                    <section class="space-y-6">
                        <h3 class="text-[10px] font-black text-gray-400 uppercase tracking-[0.3em] px-2 flex items-center gap-3">
                            <span class="w-1.5 h-1.5 bg-black rounded-full"></span>
                            "Distribución Diaria"
                        </h3>
                        <Card class="p-8 bg-black text-white rounded-[2.5rem] shadow-2xl relative overflow-hidden group border-none min-h-[400px]">
                            <div class="absolute top-0 right-0 w-32 h-32 bg-[#D4AF37]/10 -mr-16 -mt-16 rounded-full blur-2xl"></div>
                            
                            <div class="space-y-8 relative z-10">
                                <Suspense fallback=move || view! { <Loading /> }>
                                    {move || {
                                        let content: AnyView = match stats_resource.get() {
                                            Some(Ok(s)) => {
                                                if s.meal_distribution.is_empty() {
                                                    view! { <div class="text-gray-600 font-black uppercase tracking-widest py-20 text-center text-[10px]">"Sin datos en calendario"</div> }.into_any()
                                                } else {
                                                    let mut items: Vec<_> = s.meal_distribution.into_iter().collect();
                                                    items.sort_by(|a, b| b.1.cmp(&a.1));
                                                    let total_entries: usize = items.iter().map(|i| i.1).sum();

                                                    view! {
                                                        <div class="space-y-8">
                                                            {items.into_iter().map(|(label, count)| {
                                                                let percentage = (count as f32 / total_entries as f32 * 100.0) as u32;
                                                                let width = format!("{}%", percentage);

                                                                view! {
                                                                    <div class="group">
                                                                        <div class="flex justify-between items-end mb-3 px-1">
                                                                            <span class="font-black text-[10px] uppercase tracking-[0.2em] text-[#D4AF37]">{label}</span>
                                                                            <span class="text-xl font-black text-white tracking-tighter leading-none">{percentage}"%"</span>
                                                                        </div>
                                                                        <div class="h-3 w-full bg-white/5 rounded-full overflow-hidden border border-white/5 p-0.5">
                                                                            <div class="h-full bg-white rounded-full transition-all duration-1000 group-hover:bg-[#D4AF37]" style:width=width></div>
                                                                        </div>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_any()
                                                }
                                            },
                                            _ => view! { <div /> }.into_any()
                                        };
                                        content
                                    }}
                                </Suspense>
                            </div>
                        </Card>
                    </section>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MetricCard(
    label: &'static str, 
    value: String, 
    icon: &'static str,
    #[prop(default = false)] is_gold: bool
) -> impl IntoView {
    view! {
        <Card class=format!("p-8 rounded-[2rem] bg-white border border-gray-100 shadow-xl shadow-black/5 group hover:shadow-2xl transition-all duration-500 {}", 
            if is_gold { "border-b-4 border-b-[#D4AF37]" } else { "" }
        )>
            <div class="flex flex-col gap-4">
                <div class="flex items-center justify-between">
                    <span class="text-[9px] font-black text-gray-400 uppercase tracking-widest group-hover:text-black transition-colors">
                        {label}
                    </span>
                    <span class=format!("text-lg grayscale group-hover:grayscale-0 transition-all {}", if is_gold { "text-[#D4AF37] grayscale-0" } else { "" })>
                        {icon}
                    </span>
                </div>
                <div class="text-4xl font-black text-black tracking-tighter leading-none group-hover:scale-105 origin-left transition-transform duration-500">
                    {value}
                </div>
            </div>
        </Card>
    }
}

