use crate::components::ui::{Card, Loading};
use crate::tauri_bridge::{get_ingredient_trends, get_statistics};
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    let stats_resource = LocalResource::new(move || async move { get_statistics().await });
    let trends_resource = LocalResource::new(move || async move { get_ingredient_trends().await });

    view! {
        <div class="max-w-3xl mx-auto px-4 py-8 animate-fade-in space-y-8">
            <header>
                <span class="inline-block px-3 py-1 rounded-lg bg-gray-100 text-gray-500 text-[10px] font-semibold uppercase tracking-widest mb-2">
                    "Analytics"
                </span>
                <h1 class="text-3xl font-bold text-black tracking-tight">
                    "Dashboard"
                </h1>
                <p class="text-gray-500">"Tus hábitos y progreso en tiempo real."</p>
            </header>

            <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                <Suspense fallback=move || view! { <Loading size="h-4 w-4" /> }>
                    {move || stats_resource.get().map(|res| match res {
                        Ok(s) => view! {
                            <MetricCard
                                label="Total Planes"
                                value=s.total_plans.to_string()
                                icon="📋"
                            />
                            <MetricCard
                                label="Favoritos"
                                value=s.favorite_plans.to_string()
                                icon="❤️"
                            />
                            <MetricCard
                                label="Recetas"
                                value=s.recipes_count.to_string()
                                icon="🍳"
                            />
                            <MetricCard
                                label="Ingredientes"
                                value=s.ingredients_count.to_string()
                                icon="🌿"
                            />
                        }.into_any(),
                        Err(_) => view! { <div class="col-span-full py-8 text-center text-red-400 text-xs font-bold uppercase">"Error cargando estadísticas"</div> }.into_any()
                    })}
                </Suspense>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                // Trends Section
                <Card class="h-full">
                    <h3 class="text-lg font-bold text-black mb-6 flex items-center gap-2">
                        "Top Ingredientes"
                    </h3>

                    <div class="space-y-4">
                        <Suspense fallback=move || view! { <Loading /> }>
                            {move || trends_resource.get().map(|res| match res {
                                Ok(trends) => {
                                    if trends.is_empty() {
                                        return view! { <div class="text-gray-400 italic py-8 text-center text-xs">"Sin datos suficientes"</div> }.into_any();
                                    }
                                    let max_count = trends.first().map(|t| t.count).unwrap_or(1) as f32;

                                    trends.into_iter().take(5).map(|t| {
                                        let percentage = (t.count as f32 / max_count * 100.0).max(10.0);
                                        let width = format!("{}%", percentage);
                                        view! {
                                            <div class="group">
                                                <div class="flex justify-between items-end text-xs mb-1.5">
                                                    <span class="font-medium text-gray-600 group-hover:text-black transition-colors">{t.name}</span>
                                                    <span class="text-[10px] font-bold text-gray-400">{t.count}</span>
                                                </div>
                                                <div class="h-2 w-full bg-gray-100 rounded-full overflow-hidden">
                                                    <div class="h-full bg-black rounded-full transition-all duration-1000 ease-out opacity-80 group-hover:opacity-100" style:width=width></div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                },
                                Err(_) => view! { <div class="text-red-400 text-xs text-center">"Error"</div> }.into_any()
                            })}
                        </Suspense>
                    </div>
                </Card>

                // Distribution Section
                <Card class="h-full">
                    <h3 class="text-lg font-bold text-black mb-6 flex items-center gap-2">
                        "Distribución"
                    </h3>

                    <div class="space-y-6">
                        <Suspense fallback=move || view! { <Loading /> }>
                            {move || stats_resource.get().map(|res| match res {
                                Ok(s) => {
                                    if s.meal_distribution.is_empty() {
                                        return view! { <div class="text-gray-400 italic py-8 text-center text-xs">"Sin datos en calendario"</div> }.into_any();
                                    }

                                    let mut items: Vec<_> = s.meal_distribution.into_iter().collect();
                                    items.sort_by(|a, b| b.1.cmp(&a.1));
                                    let total_entries: usize = items.iter().map(|i| i.1).sum();

                                    items.into_iter().map(|(label, count)| {
                                        let percentage = (count as f32 / total_entries as f32 * 100.0) as u32;
                                        let width = format!("{}%", percentage);

                                        view! {
                                            <div class="group">
                                                <div class="flex justify-between items-end mb-2">
                                                    <span class="font-bold text-xs uppercase tracking-wider text-gray-500">{label}</span>
                                                    <span class="text-xs font-bold text-black font-mono">{percentage}"%"</span>
                                                </div>
                                                <div class="h-3 w-full bg-gray-100 rounded-lg overflow-hidden">
                                                    <div class="h-full bg-black rounded-lg transition-all duration-1000 opacity-60 group-hover:opacity-100" style:width=width></div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                },
                                Err(_) => view! { <div class="text-red-400 text-xs">"Error"</div> }.into_any()
                            })}
                        </Suspense>
                    </div>
                </Card>
            </div>
        </div>
    }
}

#[component]
fn MetricCard(label: &'static str, value: String, icon: &'static str) -> impl IntoView {
    view! {
        <Card class="p-4 hover:-translate-y-1 transition-transform cursor-default">
            <div class="flex justify-between items-start mb-2">
                <span class="text-[10px] font-bold text-gray-400 uppercase tracking-widest">
                    {label}
                </span>
                <span class="text-lg opacity-80">
                    {icon}
                </span>
            </div>
            <div class="text-2xl font-bold text-black tracking-tight">
                {value}
            </div>
        </Card>
    }
}
