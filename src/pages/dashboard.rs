use crate::components::ui::{Card, Loading};
use crate::tauri_bridge::{get_ingredient_trends, get_statistics};
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    let stats_resource = LocalResource::new(move || async move { get_statistics().await });
    let trends_resource = LocalResource::new(move || async move { get_ingredient_trends().await });

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 animate-in fade-in duration-700">
            <header class="mb-12">
                <span class="inline-block px-4 py-1.5 rounded-full bg-blue-500/10 text-blue-400 text-[10px] font-black uppercase tracking-[0.2em] mb-4">
                    "Analytics & Performance"
                </span>
                <h2 class="text-4xl font-black text-white tracking-tighter mb-2 leading-none">
                    "TU " <span class="premium-gradient-text">"DASHBOARD"</span>
                </h2>
                <div class="h-1.5 w-12 bg-blue-500 rounded-full mb-4"></div>
                <p class="text-gray-400 font-medium">"Visualiza tus hábitos y progreso nutricional en tiempo real."</p>
            </header>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
                <Suspense fallback=move || view! { <Loading size="h-4 w-4" /> }>
                    {move || stats_resource.get().map(|res| match res {
                        Ok(s) => view! {
                            <MetricCard
                                label="Total Planes"
                                value=s.total_plans.to_string()
                                icon="📋"
                                color="green"
                                progress=75
                            />
                            <MetricCard
                                label="Favoritos"
                                value=s.favorite_plans.to_string()
                                icon="❤️"
                                color="red"
                                progress=45
                            />
                            <MetricCard
                                label="Recetas"
                                value=s.recipes_count.to_string()
                                icon="🍳"
                                color="blue"
                                progress=90
                            />
                            <MetricCard
                                label="Ingredientes"
                                value=s.ingredients_count.to_string()
                                icon="🌿"
                                color="purple"
                                progress=30
                            />
                        }.into_any(),
                        Err(_) => view! { <div class="col-span-full py-12 glass rounded-3xl text-center text-red-400 font-bold uppercase tracking-widest">"Error cargando estadísticas"</div> }.into_any()
                    })}
                </Suspense>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-5 gap-8">
                // Trends Section
                <div class="lg:col-span-3">
                    <Card class="p-8 h-full glass rounded-[2.5rem] border-white/5 relative overflow-hidden">
                        <div class="absolute top-0 right-0 p-8 opacity-10 text-6xl">"📈"</div>
                        <h3 class="text-xl font-black text-white mb-8 flex items-center gap-3 tracking-tighter uppercase">
                            <div class="w-8 h-8 rounded-xl bg-green-500/20 flex items-center justify-center">
                                <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" /></svg>
                            </div>
                            "Top Ingredientes Utilizados"
                        </h3>

                        <div class="space-y-6">
                            <Suspense fallback=move || view! { <Loading /> }>
                                {move || trends_resource.get().map(|res| match res {
                                    Ok(trends) => {
                                        if trends.is_empty() {
                                            return view! { <div class="text-gray-500 italic py-12 text-center border-2 border-dashed border-white/5 rounded-3xl font-bold uppercase tracking-widest text-xs">"No hay datos suficientes aún"</div> }.into_any();
                                        }
                                        let max_count = trends.first().map(|t| t.count).unwrap_or(1) as f32;

                                        trends.into_iter().take(6).map(|t| {
                                            let percentage = (t.count as f32 / max_count * 100.0).max(10.0);
                                            let width = format!("{}%", percentage);
                                            view! {
                                                <div class="group">
                                                    <div class="flex justify-between items-end text-sm mb-2 px-1">
                                                        <span class="font-black text-gray-400 group-hover:text-green-400 transition-colors uppercase tracking-widest text-[10px]">{t.name}</span>
                                                        <span class="text-[10px] font-black text-white bg-white/5 px-2 py-0.5 rounded-lg group-hover:bg-green-500 group-hover:text-gray-900 transition-all font-mono">{t.count} " USOS"</span>
                                                    </div>
                                                    <div class="h-3 w-full bg-white/5 rounded-full overflow-hidden p-[2px]">
                                                        <div class="h-full bg-gradient-to-r from-green-600 to-green-400 rounded-full transition-all duration-1000 ease-out shadow-[0_0_15px_rgba(34,197,94,0.3)] group-hover:from-green-500 group-hover:to-green-300" style:width=width></div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>().into_any()
                                    },
                                    Err(_) => view! { <div class="text-red-400 p-8 glass rounded-2xl text-center">"Error cargando tendencias"</div> }.into_any()
                                })}
                            </Suspense>
                        </div>
                    </Card>
                </div>

                // Distribution Section
                <div class="lg:col-span-2">
                    <Card class="p-8 h-full glass rounded-[2.5rem] border-white/5 relative overflow-hidden">
                        <div class="absolute top-0 right-0 p-8 opacity-10 text-6xl">"🥧"</div>
                        <h3 class="text-xl font-black text-white mb-8 flex items-center gap-3 tracking-tighter uppercase">
                            <div class="w-8 h-8 rounded-xl bg-blue-500/20 flex items-center justify-center">
                                <svg class="w-4 h-4 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 3.055A9.001 9.001 0 1020.945 13H11V3.055z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.488 9H15V3.512A9.025 9.025 0 0120.488 9z" /></svg>
                            </div>
                            "Distribución"
                        </h3>

                        <div class="space-y-8">
                            <Suspense fallback=move || view! { <Loading /> }>
                                {move || stats_resource.get().map(|res| match res {
                                    Ok(s) => {
                                        if s.meal_distribution.is_empty() {
                                            return view! { <div class="text-gray-500 italic py-12 text-center border-2 border-dashed border-white/5 rounded-3xl font-bold uppercase tracking-widest text-xs">"No hay datos en el calendario"</div> }.into_any();
                                        }

                                        let mut items: Vec<_> = s.meal_distribution.into_iter().collect();
                                        items.sort_by(|a, b| b.1.cmp(&a.1));
                                        let total_entries: usize = items.iter().map(|i| i.1).sum();

                                        items.into_iter().map(|(label, count)| {
                                            let percentage = (count as f32 / total_entries as f32 * 100.0) as u32;
                                            let width = format!("{}%", percentage);

                                            let (bg, text, _shadow) = match label.as_str() {
                                                "Desayuno" => ("from-yellow-600 to-yellow-400", "text-yellow-400", "shadow-yellow-500/20"),
                                                "Almuerzo" => ("from-blue-600 to-blue-400", "text-blue-400", "shadow-blue-500/20"),
                                                "Cena" => ("from-purple-600 to-purple-400", "text-purple-400", "shadow-purple-500/20"),
                                                _ => ("from-gray-600 to-gray-400", "text-gray-400", "shadow-gray-500/20")
                                            };

                                            view! {
                                                <div class="group">
                                                    <div class="flex justify-between items-end mb-3">
                                                        <span class=format!("font-black uppercase tracking-widest text-[10px] {}", text)>{label}</span>
                                                        <span class="text-xs font-black text-white font-mono">{percentage}"%"</span>
                                                    </div>
                                                    <div class="h-5 w-full bg-white/5 rounded-xl overflow-hidden p-[3px] border border-white/5">
                                                        <div class=format!("h-full bg-gradient-to-r rounded-lg transition-all duration-1000 shadow-lg {}", bg) style:width=width></div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>().into_any()
                                    },
                                    Err(_) => view! { <div class="text-red-400">"Error"</div> }.into_any()
                                })}
                            </Suspense>
                        </div>
                    </Card>
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
    color: &'static str,
    progress: u32,
) -> impl IntoView {
    let (bg_gradient, shadow_color, accent_color) = match color {
        "green" => (
            "from-green-600/20 to-green-900/10",
            "shadow-green-500/10",
            "bg-green-500",
        ),
        "red" => (
            "from-red-600/20 to-red-900/10",
            "shadow-red-500/10",
            "bg-red-500",
        ),
        "blue" => (
            "from-blue-600/20 to-blue-900/10",
            "shadow-blue-500/10",
            "bg-blue-500",
        ),
        "purple" => (
            "from-purple-600/20 to-purple-900/10",
            "shadow-purple-500/10",
            "bg-purple-500",
        ),
        _ => (
            "from-gray-600/20 to-gray-900/10",
            "shadow-gray-500/10",
            "bg-gray-500",
        ),
    };

    view! {
        <Card class=format!("p-6 glass rounded-[2rem] border-white/5 relative overflow-hidden group hover:scale-[1.02] transition-transform duration-300 shadow-2xl {}", shadow_color)>
            <div class=format!("absolute inset-0 bg-gradient-to-br opacity-50 {}", bg_gradient)></div>
            <div class="relative z-10">
                <div class="flex justify-between items-start mb-4">
                    <span class="text-[10px] font-black text-gray-400 uppercase tracking-[0.2em] group-hover:text-white transition-colors">
                        {label}
                    </span>
                    <span class="text-2xl grayscale group-hover:grayscale-0 transition-all duration-500 scale-110">
                        {icon}
                    </span>
                </div>
                <div class="text-4xl font-black text-white tracking-tighter mb-4 group-hover:scale-110 transition-transform origin-left">
                    {value}
                </div>
                <div class="h-1.5 w-full bg-white/10 rounded-full overflow-hidden">
                     <div class=format!("h-full {} transition-all duration-1000", accent_color) style:width=format!("{}%", progress)></div>
                </div>
            </div>
        </Card>
    }
}
