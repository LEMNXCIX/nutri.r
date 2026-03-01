use crate::tauri_bridge::{
    calculate_nutrition, get_calendar_range, get_index, get_water_intake, update_water_intake,
    MealType,
};
use chrono::Local;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    let today = Local::now().date_naive().to_string();

    // Lógica de Hidratación
    let (water_current, set_water_current) = signal(0.0f32);
    let (water_target, set_water_target) = signal(2.5f32);

    Effect::new({
        let today = today.clone();
        move |_| {
            let today = today.clone();
            spawn_local(async move {
                if let Ok(record) = get_water_intake(today).await {
                    set_water_current.set(record.current);
                    set_water_target.set(record.target);
                }
            });
        }
    });

    let on_add_water = move |_| {
        let today = today.clone();
        let new_value = (water_current.get() + 0.25f32).min(5.0f32);
        set_water_current.set(new_value);
        spawn_local(async move {
            let _ = update_water_intake(today, new_value, water_target.get()).await;
        });
    };

    let on_remove_water = move |_| {
        let today = today.clone();
        let new_value = (water_current.get() - 0.25f32).max(0.0f32);
        set_water_current.set(new_value);
        spawn_local(async move {
            let _ = update_water_intake(today, new_value, water_target.get()).await;
        });
    };

    // Recursos de Datos
    let today_resource = LocalResource::new(move || async move {
        let today = Local::now().date_naive().to_string();
        get_calendar_range(today.clone(), today)
            .await
            .unwrap_or_default()
    });

    let plans_resource =
        LocalResource::new(move || async move { get_index().await.unwrap_or_default() });

    // Estadísticas y Comidas Computadas
    let (daily_stats, set_daily_stats) = signal((0.0, 0.0, 0.0, 0.0));
    let (meal_details, set_meal_details) = signal(Vec::<(String, String, f32)>::new());

    Effect::new(move |_| {
        if let Some(entries) = today_resource.get() {
            spawn_local(async move {
                let mut total_cal = 0.0;
                let mut total_prot = 0.0;
                let mut total_carbs = 0.0;
                let mut total_fat = 0.0;
                let mut meals = Vec::new();

                for entry in entries {
                    if let Ok(nutrition) = calculate_nutrition(&entry.plan_id).await {
                        total_cal += nutrition.total_calories;
                        total_prot += nutrition.total_protein;
                        total_carbs += nutrition.total_carbs;
                        total_fat += nutrition.total_fat;

                        let title = match entry.meal_type {
                            MealType::Breakfast => "Desayuno",
                            MealType::Lunch => "Almuerzo",
                            MealType::Dinner => "Cena",
                            MealType::Snack => "Aperitivo",
                        };

                        meals.push((
                            title.to_string(),
                            entry.plan_id.clone(),
                            nutrition.total_calories,
                        ));
                    }
                }
                set_daily_stats.set((total_cal, total_prot, total_carbs, total_fat));
                set_meal_details.set(meals);
            });
        }
    });

    view! {
        <div class="w-full font-sans pb-32 animate-in fade-in duration-700">
            // -- HERO HEADER --
            <header class="bg-white dark:bg-background-dark border-b border-neutral-100 dark:border-neutral-800 pt-16 pb-12 px-6">
                <div class="max-w-5xl mx-auto space-y-8">
                    <div class="flex items-center gap-3">
                        <span class="h-[1px] w-8 bg-black dark:bg-white transition-all"></span>
                        <span class="text-[10px] font-black text-black dark:text-white tracking-[0.4em] uppercase">"Protocolo de Nutrición"</span>
                    </div>

                    <div class="flex flex-col md:flex-row md:items-end justify-between gap-8">
                        <h1 class="text-7xl md:text-8xl font-black text-black dark:text-white tracking-tighter leading-[0.8] uppercase">
                            "Estado" <br/> "Diario"
                        </h1>
                        <div class="flex flex-col items-start md:items-end gap-2 text-right">
                            <span class="text-4xl font-black text-black dark:text-white tabular-nums tracking-tighter uppercase">
                                {move || chrono::Local::now().format("%d / %m").to_string()}
                            </span>
                            <span class="text-[9px] font-bold text-neutral-400 dark:text-neutral-500 uppercase tracking-widest">{move || chrono::Local::now().format("%A / %Y").to_string()}</span>
                        </div>
                    </div>
                </div>
            </header>

            <div class="max-w-5xl mx-auto px-6 space-y-20">
                // -- HYDRATION CONTROL --
                <section class="mt-12 bg-accent dark:bg-accent p-8 md:p-12 brutalist-border shadow-brutalist relative overflow-hidden group">
                    <div class="absolute top-0 right-0 w-64 h-64 bg-black/5 -mr-32 -mt-32 rounded-full blur-3xl group-hover:bg-black/10 transition-all duration-700"></div>

                    <div class="relative z-10 flex flex-col md:flex-row md:items-center justify-between gap-12">
                        <div class="space-y-4">
                            <div class="flex items-center gap-3">
                                <span class="material-symbols-outlined !text-base">"water_drop"</span>
                                <span class="text-[10px] font-black uppercase tracking-[0.3em]">"Integridad de Hidratación"</span>
                            </div>
                            <h2 class="text-4xl font-black text-black tracking-tighter uppercase">"Rastreador de Agua"</h2>
                        </div>

                        <div class="flex items-center gap-8">
                            <div class="flex flex-col items-center gap-1">
                                <span class="text-5xl font-black text-black tracking-tighter tabular-nums leading-none">
                                    {move || water_current.get()}
                                </span>
                                <span class="text-[10px] font-black uppercase tracking-[0.2em] opacity-40">"Vasos Registrados"</span>
                            </div>

                            <div class="flex gap-2">
                                <button
                                    on:click=on_remove_water
                                    class="w-14 h-14 bg-black text-white flex items-center justify-center hover:bg-neutral-800 transition-all active:translate-y-1 shadow-brutalist-sm"
                                >
                                    <span class="material-symbols-outlined">"remove"</span>
                                </button>
                                <button
                                    on:click=on_add_water
                                    class="w-14 h-14 bg-white text-black flex items-center justify-center hover:bg-neutral-100 transition-all active:translate-y-1 shadow-brutalist-sm"
                                >
                                    <span class="material-symbols-outlined">"add"</span>
                                </button>
                            </div>
                        </div>
                    </div>
                </section>

                // -- NUTRITION METRICS --
                <section class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div class="space-y-6">
                        <h3 class="text-[10px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-[0.4em] pl-1">"Lectura Biométrica"</h3>
                        <div class="bg-white dark:bg-neutral-900 brutalist-border dark:border-neutral-800 p-8 shadow-brutalist space-y-10">
                            <Suspense fallback=move || view! { <div class="animate-pulse space-y-8">{(0..4).map(|_| view! { <div class="h-12 bg-neutral-100 dark:bg-neutral-800"></div> }).collect_view()}</div> }>
                                {move || {
                                    let stats = daily_stats.get();
                                    view! {
                                        <div class="space-y-10">
                                            <MetricLine label="Energía Total" value=format!("{:.0}", stats.0) unit="Kcal" />
                                            <MetricLine label="Proteínas" value=format!("{:.0}", stats.1) unit="G" />
                                            <MetricLine label="Carbohidratos" value=format!("{:.0}", stats.2) unit="G" />
                                            <MetricLine label="Lípidos" value=format!("{:.0}", stats.3) unit="G" />
                                        </div>
                                    }
                                }}
                            </Suspense>
                        </div>
                    </div>

                    // -- MEAL LOG --
                    <div class="space-y-6">
                        <h3 class="text-[10px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-[0.4em] pl-1">"Registro Alimentario"</h3>
                        <div class="space-y-4">
                            <Suspense fallback=move || view! { <div class="animate-pulse space-y-4">{(0..4).map(|_| view! { <div class="h-24 bg-neutral-100 dark:bg-neutral-800 brutalist-border"></div> }).collect_view()}</div> }>
                                {move || {
                                    let meals = meal_details.get();
                                    if meals.is_empty() {
                                        view! {
                                            <div class="py-20 bg-neutral-50 dark:bg-neutral-900 border border-dashed border-neutral-200 dark:border-neutral-800 text-center space-y-4">
                                                <span class="material-symbols-outlined text-neutral-200 dark:text-neutral-700 !text-4xl">"restaurant"</span>
                                                <p class="text-[10px] font-black text-neutral-300 dark:text-neutral-600 uppercase tracking-widest leading-relaxed">"No se han detectado <br/> ingestas hoy"</p>
                                            </div>
                                        }.into_any()
                                    } else {
                                        meals.into_iter().map(|(label, id, cal)| {
                                            view! {
                                                <A href=format!("/plan/{}", id) attr:class="p-6 bg-white dark:bg-neutral-900 brutalist-border dark:border-neutral-800 flex justify-between items-center group hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-colors">
                                                    <div class="space-y-1">
                                                        <span class="text-[9px] font-black text-accent uppercase tracking-widest">{label}</span>
                                                        <h4 class="text-lg font-black text-black dark:text-white tracking-tighter uppercase leading-none">{id.chars().take(12).collect::<String>()}</h4>
                                                        <span class="text-[9px] font-bold text-neutral-400 dark:text-neutral-500 uppercase tracking-widest">{format!("{:.0} Kcal", cal)}</span>
                                                    </div>
                                                    <span class="material-symbols-outlined text-neutral-200 dark:text-neutral-700 group-hover:text-black dark:group-hover:text-white transition-colors">"chevron_right"</span>
                                                </A>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </Suspense>
                        </div>
                    </div>
                </section>

                // -- QUICK ACTIONS / CTA --
                <section class="pb-20">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <A href="/plan" attr:class="p-8 bg-black dark:bg-white text-white dark:text-black brutalist-border flex flex-col items-center gap-4 group hover:bg-neutral-900 dark:hover:bg-neutral-100 transition-all">
                            <span class="material-symbols-outlined !text-3xl group-hover:scale-110 transition-transform">"auto_awesome"</span>
                            <span class="text-[11px] font-black uppercase tracking-[0.3em]">"Generar Nuevo Plan"</span>
                        </A>
                        <A href="/pantry" attr:class="p-8 bg-white dark:bg-neutral-900 text-black dark:text-white brutalist-border flex flex-col items-center gap-4 group hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-all">
                            <span class="material-symbols-outlined !text-3xl group-hover:scale-110 transition-transform">"inventory_2"</span>
                            <span class="text-[11px] font-black uppercase tracking-[0.3em]">"Gestionar Despensa"</span>
                        </A>
                    </div>
                </section>
            </div>
        </div>
    }
}

#[component]
fn MetricLine(label: &'static str, value: String, unit: &'static str) -> impl IntoView {
    view! {
        <div class="group">
            <div class="flex justify-between items-end mb-2 px-1">
                <span class="text-[10px] font-black text-neutral-400 dark:text-neutral-500 uppercase tracking-widest group-hover:text-black dark:group-hover:text-white transition-colors">{label}</span>
                <div class="flex items-baseline gap-1">
                    <span class="text-3xl font-black text-black dark:text-white tracking-tighter tabular-nums leading-none transition-transform group-hover:scale-105 origin-right">{value}</span>
                    <span class="text-[9px] font-black text-neutral-300 dark:text-neutral-600 uppercase">{unit}</span>
                </div>
            </div>
            <div class="h-[1px] w-full bg-neutral-100 dark:bg-neutral-800 relative overflow-hidden">
                <div class="absolute inset-0 bg-accent translate-x-[-100%] group-hover:translate-x-0 transition-transform duration-700"></div>
            </div>
        </div>
    }
}
