use crate::plan_display::{format_plan_created_at, plan_display_name};
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

    // Water Logic
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

    let on_add_water = Callback::new({
        let today = today.clone();
        move |_| {
            let today = today.clone();
            let new_value = (water_current.get() + 0.25f32).min(5.0f32);
            set_water_current.set(new_value);
            spawn_local(async move {
                let _ = update_water_intake(today, new_value, water_target.get()).await;
            });
        }
    });

    let on_remove_water = Callback::new({
        let today = today.clone();
        move |_| {
            let today = today.clone();
            let new_value = (water_current.get() - 0.25f32).max(0.0f32);
            set_water_current.set(new_value);
            spawn_local(async move {
                let _ = update_water_intake(today, new_value, water_target.get()).await;
            });
        }
    });

    // Data Resources
    let today_resource = LocalResource::new(move || async move {
        let today = Local::now().date_naive().to_string();
        get_calendar_range(today.clone(), today)
            .await
            .unwrap_or_default()
    });

    let plans_resource =
        LocalResource::new(move || async move { get_index().await.unwrap_or_default() });

    // Computed Stats & Meals
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
                            MealType::Snack => "Merienda",
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
        <div class="w-full font-sans pb-32">
            // -- SECCIÓN DE TÍTULO --
            <section class="px-6 pt-8 pb-10">
                <h1 class="text-[72px] font-extrabold uppercase leading-[0.9] tracking-tighter mb-4 text-header dark:text-white break-words">
                    "Métricas" <br/> "Diarias"
                </h1>
                <div class="flex items-center gap-2 text-[10px] font-bold uppercase tracking-widest text-neutral-400">
                    <span>{Local::now().format("Ciclo %d.%m").to_string()}</span>
                    <span class="w-1 h-1 bg-neutral-300 rounded-full"></span>
                </div>
            </section>

            // -- METRICS GRID --
            <section class="px-6">
                {move || {
                    let (cal, prot, carbs, fat) = daily_stats.get();
                    view! {
                        <div class="grid grid-cols-2 gap-y-12 gap-x-8">
                            <div class="flex flex-col gap-1">
                                <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-500">"01 / Calorías"</span>
                                <div class="hairline-divider mb-2"></div>
                                <div class="flex items-baseline gap-1">
                                    <span class="text-4xl font-light tracking-tighter">{format!("{:.0}", cal)}</span>
                                    <span class="text-xs font-bold uppercase text-neutral-400">"kcal"</span>
                                </div>
                                <div class="flex items-center gap-1 mt-1">
                                    <span class="material-symbols-outlined text-accent text-sm leading-none">"trending_up"</span>
                                    <span class="text-[10px] font-bold text-accent tracking-tighter">{format!("+12.4% vs Promedio")}</span>
                                </div>
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-500">"02 / Proteína"</span>
                                <div class="hairline-divider mb-2"></div>
                                <div class="flex items-baseline gap-1">
                                    <span class="text-4xl font-light tracking-tighter">{format!("{:.0}", prot)}</span>
                                    <span class="text-xs font-bold uppercase text-neutral-400">"g"</span>
                                </div>
                                <div class="flex items-center gap-1 mt-1">
                                    <span class="material-symbols-outlined text-red-500 text-sm leading-none">"trending_down"</span>
                                    <span class="text-[10px] font-bold text-red-500 tracking-tighter">"-5.0% vs Meta"</span>
                                </div>
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-500">"03 / Carbohidratos"</span>
                                <div class="hairline-divider mb-2"></div>
                                <div class="flex items-baseline gap-1">
                                    <span class="text-4xl font-light tracking-tighter">{format!("{:.0}", carbs)}</span>
                                    <span class="text-xs font-bold uppercase text-neutral-400">"g"</span>
                                </div>
                                <div class="mt-1">
                                    <span class="px-1.5 py-0.5 border border-neutral-200 text-[9px] font-bold uppercase tracking-tighter">"Meta Alcanzada"</span>
                                </div>
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-500">"04 / Grasas Totales"</span>
                                <div class="hairline-divider mb-2"></div>
                                <div class="flex items-baseline gap-1">
                                    <span class="text-4xl font-light tracking-tighter">{format!("{:.0}", fat)}</span>
                                    <span class="text-xs font-bold uppercase text-neutral-400">"g"</span>
                                </div>
                                <div class="mt-1">
                                    <span class="bg-accent px-1.5 py-0.5 text-[9px] font-bold uppercase tracking-tighter">"Eficiencia Optimizada"</span>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </section>

            // -- HIDRATACIÓN Y PROGRESO --
            <section class="px-6 py-16 space-y-12">
                <div>
                    <div class="flex justify-between items-end mb-3">
                        <div class="flex flex-col gap-1">
                            <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-950 dark:text-neutral-300">"Índice de Hidratación"</span>
                            <div class="flex items-center gap-2 mt-1">
                                <button on:click=move |_| on_remove_water.run(()) class="text-neutral-300 hover:text-neutral-950 dark:text-neutral-500 dark:hover:text-white">
                                    <span class="material-symbols-outlined !text-sm">"remove"</span>
                                </button>
                                <span class="text-xs font-medium tabular-nums text-neutral-950 dark:text-white">{move || format!("{:.1} / {:.1} L", water_current.get(), water_target.get())}</span>
                                <button on:click=move |_| on_add_water.run(()) class="text-neutral-300 hover:text-neutral-950 dark:text-neutral-500 dark:hover:text-white">
                                    <span class="material-symbols-outlined !text-sm">"add"</span>
                                </button>
                            </div>
                        </div>
                    </div>
                    <div class="relative w-full h-[2px] bg-neutral-100 dark:bg-neutral-800">
                        <div
                            class="absolute top-0 left-0 h-full bg-neutral-950 dark:bg-white transition-all duration-700"
                            style:width=move || format!("{}%", (water_current.get() / water_target.get() * 100.0).min(100.0))
                        ></div>
                    </div>
                </div>

                <div>
                    <div class="flex justify-between items-end mb-3">
                        <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-950 dark:text-neutral-300">"Diversidad de Micronutrientes"</span>
                        <span class="text-xs font-medium tabular-nums text-neutral-950 dark:text-white">"84%"</span>
                    </div>
                    <div class="relative w-full h-[2px] bg-neutral-100 dark:bg-neutral-800">
                        <div class="absolute top-0 left-0 h-full bg-accent" style="width: 84%;"></div>
                    </div>
                </div>
            </section>

            // -- SIGUIENTE INGESTA PROGRAMADA --
            <section class="px-6 mb-16 space-y-8">
                {move || -> AnyView {
                    let meals = meal_details.get();
                    if meals.is_empty() {
                        view! {
                            <div class="p-12 border border-neutral-100 dark:border-neutral-800 flex flex-col items-center justify-center gap-4 text-center">
                                <span class="material-symbols-outlined text-neutral-200 dark:text-neutral-700 !text-4xl">"fastfood"</span>
                                <p class="text-[10px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Sin Comidas Programadas"</p>
                                <A href="/calendar" attr:class="mt-4 px-6 py-3 bg-neutral-950 dark:bg-white text-accent text-[10px] font-bold uppercase tracking-widest hover:bg-accent hover:text-neutral-950 transition-all">"Asignar Plan"</A>
                            </div>
                        }.into_any()
                    } else {
                        meals.into_iter().map(|(title, id, cal)| {
                            view! {
                                <A href=format!("/plan/{}", id) attr:class="block relative group aspect-[4/5] overflow-hidden bg-neutral-100 dark:bg-neutral-800">
                                    <img
                                        alt=title.clone()
                                        class="w-full h-full object-cover grayscale hover:grayscale-0 transition-all duration-700"
                                        src="https://lh3.googleusercontent.com/aida-public/AB6AXuCuLIs4J3BB-Asz5cdNOorESMj1X3AVHQ_CyacDzU2zpMKJ4AmCCVsAedD5NzL-tBYxXv2eygd4hFNASqgdKD0gQnv78equgwci1mxJTvwA2XoV8I5GKSnShEzhTNk-Sfq7lK0QTcqEUsgGCWjJnyFLnU1YJVwoIJEK5Hfo3fFegV_Qf78T58vwbdtEQOflSZsT_ZYtWI8zXgmyhEojqt3UqYpvZwNrIO1VYttV3E3A3lfStG6x_jIYbQxMszgc2jS4Z_ticQKZ8Mha"
                                    />
                                    <div class="absolute inset-0 bg-neutral-950/10 dark:bg-black/30"></div>
                                    <div class="absolute top-6 left-6">
                                        <div class="bg-white dark:bg-neutral-900 px-3 py-1.5 inline-block">
                                            <p class="text-[10px] font-bold uppercase tracking-[0.2em] text-neutral-950 dark:text-white">"Siguiente Ingesta"</p>
                                        </div>
                                    </div>
                                    <div class="absolute bottom-0 left-0 right-0 p-8 bg-gradient-to-t from-neutral-950/60 to-transparent">
                                        <h3 class="text-white text-3xl font-extrabold uppercase tracking-tighter leading-none mb-2">{title.clone()}</h3>
                                        <p class="text-white/80 text-[10px] font-bold uppercase tracking-widest">{format!("{:.0} KCAL / {}", cal, id.chars().take(8).collect::<String>())}</p>
                                    </div>
                                </A>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </section>

            // -- MI ARCHIVO --
            <section class="px-6 pb-12">
                <div class="flex items-center justify-between mb-8">
                    <h2 class="text-[10px] font-bold uppercase tracking-[0.3em] text-neutral-400 dark:text-neutral-500">"Mi Archivo"</h2>
                    <A href="/add" attr:class="bg-neutral-950 dark:bg-white text-white dark:text-neutral-950 p-2 flex items-center justify-center">
                        <span class="material-symbols-outlined !text-sm">"add"</span>
                    </A>
                </div>

                <div class="space-y-4">
                    <Suspense fallback=move || view! { <div class="animate-pulse h-12 bg-neutral-50 dark:bg-neutral-900 mb-4"></div> }>
                        {move || plans_resource.get().map(|plans| {
                            plans.into_iter().take(5).map(|plan| {
                                let pid = plan.id.clone();
                                let title = plan_display_name(&plan);
                                let date = format_plan_created_at(&plan);
                                view! {
                                    <A href=format!("/plan/{}", pid) attr:class="flex items-center justify-between py-6 border-b border-neutral-100 dark:border-neutral-800 group">
                                        <div class="flex items-center gap-6">
                                            <span class="text-xs font-black text-neutral-950 dark:text-white">"P"</span>
                                            <div class="flex flex-col gap-1">
                                                <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-950 dark:text-white group-hover:text-accent dark:group-hover:text-accent transition-colors">{title}</span>
                                                <span class="text-[8px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">{date.clone()}</span>
                                            </div>
                                        </div>
                                        <span class="material-symbols-outlined text-neutral-200 dark:text-neutral-700 group-hover:text-neutral-950 dark:group-hover:text-white transition-colors">"arrow_forward"</span>
                                    </A>
                                }
                            }).collect_view()
                        })}
                    </Suspense>
                </div>
            </section>
        </div>
    }.into_any()
}
