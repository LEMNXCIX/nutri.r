use crate::components::ui::Loading;
use crate::tauri_bridge::{get_calendar_range, get_index, remove_calendar_entry, MealType};
use chrono::{Datelike, NaiveDate};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use std::collections::HashMap;

#[component]
pub fn DailyView() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let navigate_add = navigate.clone();
    let navigate_main = navigate.clone();
    let date_param = move || {
        params
            .read()
            .get("date")
            .unwrap_or_else(|| "2026-02-25".to_string())
    };

    let calendar_resource = LocalResource::new(move || {
        let date = date_param();
        async move {
            get_calendar_range(date.clone(), date)
                .await
                .unwrap_or_default()
        }
    });

    let plans_resource =
        LocalResource::new(move || async move { get_index().await.unwrap_or_default() });

    let parsed_date = move || {
        NaiveDate::parse_from_str(&date_param(), "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2026, 2, 25).unwrap())
    };

    let day_name = move || match parsed_date().weekday() {
        chrono::Weekday::Mon => "Lunes",
        chrono::Weekday::Tue => "Martes",
        chrono::Weekday::Wed => "Miércoles",
        chrono::Weekday::Thu => "Jueves",
        chrono::Weekday::Fri => "Viernes",
        chrono::Weekday::Sat => "Sábado",
        chrono::Weekday::Sun => "Domingo",
    };

    let month_name = move || match parsed_date().month() {
        1 => "Enero",
        2 => "Febrero",
        3 => "Marzo",
        4 => "Abril",
        5 => "Mayo",
        6 => "Junio",
        7 => "Julio",
        8 => "Agosto",
        9 => "Septiembre",
        10 => "Octubre",
        11 => "Noviembre",
        12 => "Diciembre",
        _ => "Desconocido",
    };

    let on_remove = move |meal: MealType| {
        let date = date_param();
        let calendar_resource = calendar_resource.clone();
        spawn_local(async move {
            if let Ok(_) = remove_calendar_entry(date, meal).await {
                calendar_resource.refetch();
            }
        });
    };

    view! {
        <div class="bg-white dark:bg-background-dark text-black dark:text-white min-h-screen">
            <header class="sticky top-0 z-50 bg-white dark:bg-background-dark border-b border-black dark:border-neutral-800 px-6 py-4 flex justify-between items-center">
                <button on:click=move |_| {
                    if let Some(window) = web_sys::window() {
                        if let Ok(history) = window.history() {
                            let _ = history.back();
                        }
                    }
                } class="flex items-center justify-center">
                    <span class="material-icons-outlined text-2xl">arrow_back</span>
                </button>
                <div class="text-[10px] tracking-[0.2em] font-black uppercase text-zinc-400">Planning Mastery</div>
                <button class="flex items-center justify-center">
                    <span class="material-icons-outlined text-2xl">more_vert</span>
                </button>
            </header>

            <main class="px-6 pb-32">
                <section class="py-8 border-b-2 border-black dark:border-neutral-800">
                    <div class="flex flex-col">
                        <span class="text-sm font-black uppercase tracking-widest text-zinc-500 dark:text-zinc-400">{move || day_name()}</span>
                        <h1 class="text-7xl font-black uppercase leading-none mt-1">{move || parsed_date().day()}</h1>
                        <div class="mt-4 flex justify-between items-end">
                            <span class="text-xs font-bold uppercase tracking-tighter text-zinc-400">{move || format!("{} {}", month_name(), parsed_date().year())}</span>
                            <div class="flex gap-2">
                                <div class="px-3 py-1 bg-black text-white dark:bg-white dark:text-black text-[10px] font-black uppercase">Óptimo</div>
                                <div class="px-3 py-1 border border-black dark:border-neutral-700 text-[10px] font-black uppercase">Activo</div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="grid grid-cols-3 gap-0 border-b border-black dark:border-neutral-800">
                    <div class="py-4 border-r border-black dark:border-neutral-800">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Calorías</div>
                        <div class="text-lg font-black">-- <span class="text-[10px] text-zinc-400 ml-1">/ 2.2k</span></div>
                    </div>
                    <div class="py-4 px-4 border-r border-black dark:border-neutral-800">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Proteína</div>
                        <div class="text-lg font-black">--</div>
                    </div>
                    <div class="py-4 px-4">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Estado</div>
                        <div class="flex items-center gap-1">
                            <div class="w-2 h-2 bg-[#00FF66]"></div>
                            <div class="text-lg font-black uppercase">92%</div>
                        </div>
                    </div>
                </section>

                <div class="mt-8 space-y-12">
                    <Suspense fallback=move || view! { <div class="flex justify-center p-10"><Loading /></div> }>
                        {move || {
                            let entries = calendar_resource.get().unwrap_or_default();
                            let plans = plans_resource.get().unwrap_or_default();

                            let mut meal_map = HashMap::new();
                            for entry in entries {
                                meal_map.insert(entry.meal_type, entry.plan_id);
                            }

                            vec![
                                (MealType::Breakfast, "Desayuno", "07:30 AM"),
                                (MealType::Lunch, "Almuerzo", "01:15 PM"),
                                (MealType::Dinner, "Cena", "08:00 PM"),
                                (MealType::Snack, "Snack", "04:30 PM"),
                            ].into_iter().map(|(m_type, label, time)| {
                                let plan_id = meal_map.get(&m_type).cloned();
                                let plan_info = plan_id.as_ref().and_then(|id| {
                                    plans.iter().find(|p| p.id == *id)
                                });

                                let description = if let (Some(_info), Some(p_id)) = (plan_info, plan_id.as_ref()) {
                                    // Try to match the meal description from weekly_structure if available
                                    // But we don't know the day_index for this specific date easily without more info
                                    // For now, let's just show the Plan ID or a generic "Assigned"
                                    // In a real scenario we'd need to know which day of the plan this is.
                                    // Let's assume day_index is derived from the creation date vs this date?
                                    // Actually, let's just show "Plan: [ID]" for now.
                                    format!("Plan: {}", &p_id[..8.min(p_id.len())])
                                } else {
                                    "No asignado".to_string()
                                };

                                let nav_detail = navigate_main.clone();
                                let nav_assign = navigate_main.clone();
                                let p_id_for_nav = plan_id.clone();
                                let meal_type_for_remove = m_type;

                                view! {
                                    <section>
                                        <div class="flex justify-between items-baseline mb-4">
                                            <h2 class="text-3xl font-black italic uppercase tracking-tighter">{label}</h2>
                                            <span class="text-xs font-bold text-zinc-400">{time}</span>
                                        </div>
                                        <div class="space-y-4">
                                            <div class="flex items-center gap-4 group">
                                                <div class="w-16 h-16 bg-zinc-100 dark:bg-neutral-900 overflow-hidden border border-zinc-200 dark:border-neutral-800 flex-shrink-0 flex items-center justify-center">
                                                    <span class="material-icons-outlined text-zinc-400">restaurant</span>
                                                </div>
                                                <div class="flex-grow">
                                                    <div class="flex justify-between items-start">
                                                        <div
                                                            on:click=move |_| {
                                                                if let Some(id) = p_id_for_nav.as_ref() {
                                                                    nav_detail(&format!("/plan/{}", id), Default::default());
                                                                }
                                                            }
                                                            class="cursor-pointer"
                                                        >
                                                            <h3 class="font-bold text-sm uppercase">{description}</h3>
                                                            {if let Some(info) = plan_info {
                                                                view! {
                                                                    <p class="text-[10px] text-zinc-500 mt-0.5">
                                                                        {info.proteinas.join(", ")}
                                                                    </p>
                                                                }.into_any()
                                                            } else {
                                                                ().into_any()
                                                            }}
                                                        </div>
                                                        <div class="flex gap-2">
                                                            {if plan_id.is_some() {
                                                                view! {
                                                                    <button
                                                                        on:click=move |_| on_remove(meal_type_for_remove)
                                                                        class="text-zinc-300 hover:text-red-500 transition-colors"
                                                                    >
                                                                        <span class="material-icons-outlined text-xl">delete_outline</span>
                                                                    </button>
                                                                    <span class="material-icons-outlined text-[#00FF66] text-xl">check_circle</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <button
                                                                        on:click=move |_| nav_assign("/calendar", Default::default())
                                                                        class="text-zinc-300 hover:text-primary transition-colors"
                                                                    >
                                                                        <span class="material-icons-outlined text-xl">add_circle_outline</span>
                                                                    </button>
                                                                }.into_any()
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="mt-6 h-[1px] bg-zinc-200 dark:bg-neutral-800"></div>
                                    </section>
                                }
                            }).collect_view()
                        }}
                    </Suspense>
                </div>

                <button
                    on:click=move |_| navigate_add("/add", Default::default())
                    class="mt-12 w-full bg-black dark:bg-white text-white dark:text-black py-5 font-black uppercase tracking-[0.3em] text-sm"
                >
                    Generar Nuevo Plan
                </button>
            </main>
        </div>
    }
}
