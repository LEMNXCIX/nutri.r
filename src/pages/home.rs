use crate::components::dashboard::{MealCard, NutritionSummary, WaterTracker};
use crate::tauri_bridge::{
    calculate_nutrition, generate_week, get_calendar_range, get_index, get_water_intake,
    update_water_intake, MealType,
};
use chrono::Local;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Home() -> impl IntoView {
    let today = Local::now().date_naive().to_string();

    // Water Logic (Persistent)
    let (water_current, set_water_current) = signal(0.0f32);
    let (water_target, set_water_target) = signal(2.5f32);

    // Load water data on mount
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
    let (daily_stats, set_daily_stats) = signal((0.0, 0.0, 0.0, 0.0)); // cal, prot, carbs, fat
    let (meal_details, set_meal_details) = signal(Vec::<(String, String, f32, String)>::new()); // title, desc, cals, icon

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

                        let (icon, title) = match entry.meal_type {
                            MealType::Breakfast => (
                                "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4",
                                "Desayuno",
                            ),
                            MealType::Lunch => (
                                "M6.125 15.604l.15-.316a5.002 5.002 0 0 1 8.928-1.558 5.002 5.002 0 0 1 2.37 5.766l-.1.353a1.5 1.5 0 0 1-1.442 1.151H8.056a1.5 1.5 0 0 1-1.39-4.821l.156-.258-.696-.317z M12 3v9",
                                "Almuerzo",
                            ),
                            MealType::Dinner => (
                                "M14.6 9c-1.5 1.2-3.6 1.4-4.6.4s-.8-3.1.4-4.6c-2.4-.4-4.8.9-5.9 3.2C3.1 11.2 4.4 14.8 7.3 16c2.9 1.1 5.4.1 6.8-1 1.4-1.1 2.3-3.6.5-6z",
                                "Cena",
                            ),
                            MealType::Snack => (
                                "M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z",
                                "Merienda",
                            ),
                        };

                        meals.push((
                            title.to_string(),
                            entry.plan_id.clone(), // Use ID as description for now
                            nutrition.total_calories,
                            icon.to_string(),
                        ));
                    }
                }
                set_daily_stats.set((total_cal, total_prot, total_carbs, total_fat));
                set_meal_details.set(meals);
            });
        }
    });

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 animate-in pb-32 md:pb-8">
             <header class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-14 md:mb-10">
                <div>
                     <h2 class="text-3xl font-bold text-gray-900 tracking-tight">"Dashboard"</h2>
                     <p class="text-sm text-gray-500">"Tu resumen diario."</p>
                </div>
                <div class="hidden md:block">
                     <span class="text-xs font-bold text-gray-400 uppercase tracking-wider">{Local::now().format("%A, %d %B").to_string()}</span>
                </div>
            </header>

            <div class="grid grid-cols-1 md:grid-cols-12 gap-8">
                // Left Column: Nutrition & Water (Desktop: col-span-4)
                <div class="md:col-span-12 lg:col-span-4 space-y-6 min-w-0">
                    // Nutrition Summary Section
                    {move || {
                        let (cal, prot, carbs, fat) = daily_stats.get();
                        let p_pct = if cal > 0.0 { (prot * 4.0 / cal * 100.0) as i32 } else { 0 };
                        let c_pct = if cal > 0.0 { (carbs * 4.0 / cal * 100.0) as i32 } else { 0 };
                        let f_pct = if cal > 0.0 { (fat * 9.0 / cal * 100.0) as i32 } else { 0 };

                        view! {
                            <NutritionSummary
                                calories_current=cal as i32
                                calories_target=2200
                                protein_pct=p_pct
                                carbs_pct=c_pct
                                fat_pct=f_pct
                            />
                        }
                    }}

                    // Water Section
                    <WaterTracker
                        current=water_current.into()
                        target=water_target.get()
                        on_add=on_add_water
                        on_remove=on_remove_water
                    />
                </div>

                // Middle Column: Meals (Desktop: col-span-4)
                <div class="md:col-span-6 lg:col-span-4 space-y-4 min-w-0">
                    <div class="flex items-center justify-between px-1 h-6">
                        <h2 class="text-xs font-bold text-gray-400 uppercase tracking-wider">"Comidas de Hoy"</h2>
                        <a href="/calendar" class="text-[10px] font-bold text-blue-500 hover:text-blue-600 uppercase tracking-wider">"Ver Calendario"</a>
                    </div>
                    <div class="space-y-3">
                        {move || {
                            let meals = meal_details.get();
                            if meals.is_empty() {
                               view! {
                                    <div class="card p-8 flex flex-col items-center justify-center gap-3 h-[280px]">
                                        <div class="w-12 h-12 bg-gray-50 rounded-full flex items-center justify-center text-gray-300">
                                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" /></svg>
                                        </div>
                                        <div class="text-center">
                                            <p class="text-xs text-gray-900 font-bold uppercase tracking-wider">"Sin comidas"</p>
                                            <p class="text-[10px] text-gray-400 font-medium mt-1">"No hay comidas asignadas hoy"</p>
                                        </div>
                                        <a href="/calendar" class="mt-4 px-4 py-2 bg-black text-white rounded-xl text-[10px] font-bold uppercase tracking-wider hover:bg-gray-800 transition-colors shadow-soft-lg">"ASIGNAR"</a>
                                    </div>
                               }.into_any()
                            } else {
                                meals.into_iter().map(|(title, desc, cal, icon)| {
                                    view! {
                                        <MealCard
                                            title=title
                                            description=desc
                                            calories=cal as i32
                                            icon=icon
                                        />
                                    }
                                }).collect::<Vec<_>>().into_any()
                            }
                        }}
                    </div>
                </div>

                // Right Column: Plans (Desktop: col-span-4)
                <div class="md:col-span-6 lg:col-span-4 space-y-4 min-w-0">
                    <div class="flex items-center justify-between px-1 h-6">
                        <h2 class="text-xs font-bold text-gray-400 uppercase tracking-wider">"Mis Planes"</h2>
                        <div class="flex items-center gap-2">
                            <button
                                on:click=move |_| {
                                    spawn_local(async move {
                                        let _ = generate_week().await;
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.location().reload();
                                        }
                                    });
                                }
                                class="text-[10px] font-bold text-black hover:text-gray-600 uppercase tracking-wider px-2 py-1 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
                            >
                                "+ Generar"
                            </button>
                            <a href="/history" class="text-[10px] font-bold text-gray-400 hover:text-black uppercase tracking-wider">"Ver Todos"</a>
                        </div>
                    </div>
                    <div class="space-y-3">
                        <Suspense fallback=move || view! { <div class="text-center py-4 text-xs text-gray-400">"Cargando..."</div> }>
                            {move || {
                                let plans = plans_resource.get().unwrap_or_default();
                                if plans.is_empty() {
                                    let (generating, set_generating) = signal(false);
                                    let (gen_message, set_gen_message) = signal(String::new());



                                    view! {
                                        <div class="card p-8 flex flex-col items-center justify-center gap-3 h-[280px]">
                                            <div class="w-12 h-12 bg-gray-50 rounded-full flex items-center justify-center text-gray-300">
                                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" /></svg>
                                            </div>
                                            <div class="text-center">
                                                <p class="text-xs text-gray-900 font-bold uppercase tracking-wider">"Sin planes"</p>
                                                <p class="text-[10px] text-gray-400 font-medium mt-1">"Genera tu primer plan"</p>
                                            </div>
                                            <button
                                                on:click=move |_| {
                                                    set_generating.set(true);
                                                    set_gen_message.set("Generando plan...".to_string());
                                                    spawn_local(async move {
                                                        match generate_week().await {
                                                            Ok(_) => {
                                                                set_gen_message.set("¡Plan creado! Recargando...".to_string());
                                                                if let Some(window) = web_sys::window() {
                                                                    let _ = window.location().reload();
                                                                }
                                                            }
                                                            Err(e) => {
                                                                set_gen_message.set(format!("Error: {}", e));
                                                                set_generating.set(false);
                                                            }
                                                        }
                                                    });
                                                }
                                                prop:disabled=move || generating.get()
                                                class="mt-4 px-4 py-2 bg-black hover:bg-gray-900 text-white rounded-xl text-[10px] font-bold uppercase tracking-wider transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                            >
                                                {move || if generating.get() { "GENERANDO..." } else { "CREAR PLAN" }}
                                            </button>
                                            {move || if !gen_message.get().is_empty() {
                                                view! { <p class="text-[10px] text-gray-500 mt-2">{gen_message.get()}</p> }.into_any()
                                            } else {
                                                ().into_any()
                                            }}
                                        </div>
                                    }.into_any()
                                } else {
                                    plans.into_iter().take(4).map(|plan| { // Show last 4
                                        view! {
                                            <a href=format!("/plan/{}", plan.id) class="block group">
                                                <div class="bg-white p-4 rounded-2xl border border-gray-100 shadow-sm hover:shadow-soft-lg transition-all active:scale-98 flex items-center justify-between group-hover:border-gray-200">
                                                    <div>
                                                        <h3 class="font-bold text-gray-900 text-sm group-hover:text-black transition-colors">{plan.id.chars().take(20).collect::<String>()}</h3>
                                                        <div class="flex items-center gap-2 mt-1">
                                                            <span class="text-[10px] text-gray-400 font-bold uppercase tracking-wider">{plan.fecha}</span>
                                                            {if plan.is_favorite {
                                                                view! { <span class="text-[8px] bg-red-50 text-red-500 px-1.5 py-0.5 rounded-full font-bold uppercase tracking-wider">"FAV"</span> }.into_any()
                                                            } else {
                                                                ().into_any()
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="w-8 h-8 rounded-full bg-gray-50 flex items-center justify-center text-gray-400 group-hover:bg-black group-hover:text-white transition-all">
                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                                        </svg>
                                                    </div>
                                                </div>
                                            </a>
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                }
                            }}
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>

    }
}
