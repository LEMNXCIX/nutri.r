use crate::components::ui::Loading;
use crate::plan_display::{format_plan_created_at, plan_display_name};
use crate::tauri_bridge::{
    assign_weekly_plan_to_date, get_calendar_range, get_index, remove_calendar_entry,
    CalendarEntry, MealType,
};
use chrono::{Datelike, Local, Month, NaiveDate};
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use std::collections::HashMap;

#[component]
pub fn Calendar() -> impl IntoView {
    let navigate = use_navigate();
    let now = Local::now().date_naive();
    let (current_year, set_current_year) = signal(now.year());
    let (current_month, set_current_month) = signal(now.month());

    let plans = LocalResource::new(move || async move { get_index().await.unwrap_or_default() });

    let calendar_resource = LocalResource::new(move || {
        let year = current_year.get();
        let month = current_month.get();
        async move {
            let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let end = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1)
                    .unwrap()
                    .pred_opt()
                    .unwrap()
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1)
                    .unwrap()
                    .pred_opt()
                    .unwrap()
            };
            get_calendar_range(start.to_string(), end.to_string())
                .await
                .unwrap_or_default()
        }
    });

    let (show_assign_modal, set_show_assign_modal) = signal(Option::<(NaiveDate, MealType)>::None);

    let on_prev_month = move |_| {
        if current_month.get() == 1 {
            set_current_month.set(12);
            set_current_year.update(|y| *y -= 1);
        } else {
            set_current_month.update(|m| *m -= 1);
        }
    };

    let on_next_month = move |_| {
        if current_month.get() == 12 {
            set_current_month.set(1);
            set_current_year.update(|y| *y += 1);
        } else {
            set_current_month.update(|m| *m += 1);
        }
    };

    let on_assign = move |plan_id: String| {
        if let Some((date, _meal)) = show_assign_modal.get() {
            let date_str = date.to_string();
            let calendar_resource = calendar_resource.clone();
            spawn_local(async move {
                if let Ok(_) = assign_weekly_plan_to_date(&date_str, &plan_id).await {
                    calendar_resource.refetch();
                }
            });
            set_show_assign_modal.set(None);
        }
    };

    let _on_remove = move |(date, meal): (String, MealType)| {
        let calendar_resource = calendar_resource.clone();
        spawn_local(async move {
            if let Ok(_) = remove_calendar_entry(date, meal).await {
                calendar_resource.refetch();
            }
        });
    };

    view! {
        <div class="bg-white dark:bg-background-dark min-h-full font-sans text-black dark:text-white flex flex-col pb-32">
            <header class="px-6 py-6 border-b border-hairline dark:border-neutral-800">
                <div class="flex items-center justify-between mb-4">
                    <div class="flex items-center space-x-2">
                        <div class="w-8 h-[2px] bg-primary"></div>
                        <span class="text-[10px] font-black tracking-[0.3em] text-zinc-400 dark:text-zinc-500 uppercase">"Calendario"</span>
                    </div>
                    <div class="flex space-x-4">
                        <button on:click=on_prev_month class="text-black dark:text-white active:scale-90 transition-transform">
                            <span class="material-symbols-outlined text-xl">"chevron_left"</span>
                        </button>
                        <button on:click=on_next_month class="text-black dark:text-white active:scale-90 transition-transform">
                            <span class="material-symbols-outlined text-xl">"chevron_right"</span>
                        </button>
                    </div>
                </div>
                <h1 class="text-[72px] break-words font-[900] tracking-tighter uppercase leading-none">
                    {move || format!("{} {}",
                        Month::try_from(current_month.get() as u8).ok()
                            .map(|m| match m {
                                Month::January => "Enero",
                                Month::February => "Febrero",
                                Month::March => "Marzo",
                                Month::April => "Abril",
                                Month::May => "Mayo",
                                Month::June => "Junio",
                                Month::July => "Julio",
                                Month::August => "Agosto",
                                Month::September => "Septiembre",
                                Month::October => "Octubre",
                                Month::November => "Noviembre",
                                Month::December => "Diciembre",
                            })
                            .unwrap_or(""),
                        current_year.get())}
                </h1>
            </header>

            <main class="min-h-0 flex-1">
                <div class="grid grid-cols-7 border-b border-hairline dark:border-neutral-800 bg-zinc-50 dark:bg-neutral-900">
                    {vec!["Lu", "Ma", "Mi", "Ju", "Vi", "Sa", "Do"].into_iter().map(|day| view! {
                        <div class="py-3 text-center text-[10px] font-bold text-zinc-400 dark:text-zinc-500 uppercase tracking-widest">{day}</div>
                    }).collect_view()}
                </div>

                <div class="grid grid-cols-7 bg-white dark:bg-background-dark">
                    <Suspense fallback=move || view! { <div class="col-span-7 py-20 flex justify-center"><Loading /></div> }>
                        {move || {
                            let entries = calendar_resource.get().unwrap_or_default();
                            let mut entries_map = HashMap::new();
                            for e in entries {
                                entries_map.entry(e.date.clone()).or_insert_with(Vec::new).push(e);
                            }

                            let year = current_year.get();
                            let month = current_month.get();
                            let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                            let weekday = first_day.weekday().number_from_monday();

                            let days_in_month = if month == 12 {
                                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().pred_opt().unwrap().day()
                            } else {
                                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().pred_opt().unwrap().day()
                            };

                            let mut grid_items = Vec::new();
                            // Empty cells at the start
                            for _ in 1..weekday {
                                grid_items.push(view! { <div class="aspect-square hairline-border dark:border-neutral-800 opacity-30 bg-zinc-50 dark:bg-neutral-900"></div> }.into_any());
                            }

                            // Day cells
                            for d in 1..=days_in_month {
                                let date = NaiveDate::from_ymd_opt(year, month, d).unwrap();
                                let date_str = date.to_string();
                                let day_entries = entries_map.get(&date_str).cloned().unwrap_or_default();
                                let is_today = date == now;

                                let nav = navigate.clone();
                                let d_clone = d;
                                let date_str_clone = date_str.clone();

                                grid_items.push(view! {
                                    <div
                                        on:click=move |_| nav(&format!("/calendar/{}", date_str_clone), Default::default())
                                        class=format!("aspect-square hairline-border dark:border-neutral-800 p-3 flex flex-col justify-between relative cursor-pointer hover:bg-zinc-50 dark:hover:bg-neutral-800 transition-colors {}",
                                            if is_today { "bg-primary/[0.03] dark:bg-primary/[0.05]" } else { "" }
                                        )
                                    >
                                        {if is_today {
                                            view! { <div class="absolute inset-0 border-2 border-primary z-10 pointer-events-none"></div> }.into_any()
                                        } else {
                                            ().into_any()
                                        }}

                                        <span class=format!("text-xs font-black {}", if is_today { "text-black dark:text-white" } else { "text-zinc-500 dark:text-zinc-600" })>
                                            {format!("{:02}", d_clone)}
                                        </span>

                                        <div class="flex gap-1">
                                            {render_dot(&day_entries, MealType::Breakfast)}
                                            {render_dot(&day_entries, MealType::Lunch)}
                                            {render_dot(&day_entries, MealType::Dinner)}
                                        </div>
                                    </div>
                                }.into_any());
                            }

                            // Empty cells at the end to fill the row
                            let total_cells = weekday - 1 + days_in_month;
                            let remaining = if total_cells % 7 == 0 { 0 } else { 7 - (total_cells % 7) };
                            for _ in 0..remaining {
                                grid_items.push(view! { <div class="aspect-square hairline-border dark:border-neutral-800 bg-zinc-50 dark:bg-neutral-900"></div> }.into_any());
                            }

                            grid_items
                        }}
                    </Suspense>
                </div>

                <div class="p-6 flex items-center space-x-6 border-b border-hairline dark:border-neutral-800">
                    <div class="flex items-center space-x-2">
                        <div class="w-2 h-2 rounded-full bg-primary"></div>
                        <span class="text-[10px] font-bold uppercase tracking-widest text-zinc-500 dark:text-zinc-400">"Assigned"</span>
                    </div>
                </div>

                <div class="h-32"></div>
            </main>

            // Re-using the modal logic from before but could be refined if needed
            {move || if let Some((_date, _meal)) = show_assign_modal.get() {
                 let on_assign = on_assign.clone();
                view! {
                    <Portal>
                        <div class="fixed inset-0 bg-white/90 dark:bg-background-dark/90 backdrop-blur-2xl z-[500] flex items-center justify-center p-4 animate-in fade-in duration-500">
                            <div class="max-w-md w-full p-10 bg-white dark:bg-neutral-900 border border-hairline dark:border-neutral-800 shadow-2xl relative overflow-hidden text-center">
                                <div class="absolute top-0 left-0 w-full h-2 bg-primary"></div>
                                <h3 class="text-3xl font-black text-black dark:text-white mb-2 tracking-tighter uppercase italic">"Select Blueprint"</h3>
                                <div class="max-h-[400px] overflow-y-auto space-y-3 mb-10 pr-2">
                                    {move || plans.get().map(|list| {
                                        let on_assign = on_assign.clone();
                                        list.into_iter().map(|plan| {
                                            let pid = plan.id.clone();
                                            let title = plan_display_name(&plan);
                                            let created_at = format_plan_created_at(&plan);
                                            let on_assign = on_assign.clone();
                                            view! {
                                                <button
                                                    on:click=move |_| on_assign(pid.clone())
                                                    class="w-full text-left p-6 bg-zinc-50 dark:bg-neutral-800 hover:bg-primary dark:hover:bg-primary text-black dark:text-white hover:text-black dark:hover:text-black border border-hairline dark:border-neutral-700 transition-all flex items-center justify-between"
                                                >
                                                    <div class="flex flex-col gap-1">
                                                        <span class="font-black text-xs uppercase tracking-widest">{title}</span>
                                                        <span class="text-[9px] font-bold uppercase tracking-[0.2em] text-zinc-400 dark:text-zinc-500">
                                                            {created_at}
                                                        </span>
                                                    </div>
                                                    <span class="material-symbols-outlined">"add"</span>
                                                </button>
                                            }
                                        }).collect_view()
                                    })}
                                </div>
                                <button on:click=move |_| set_show_assign_modal.set(None) class="text-[10px] font-black uppercase tracking-widest text-zinc-400 dark:text-zinc-500 hover:text-black dark:hover:text-white transition-colors">"Dismiss"</button>
                            </div>
                        </div>
                    </Portal>
                }.into_any()
            } else { ().into_any() }}
        </div>
    }
}

fn render_dot(entries: &[CalendarEntry], meal: MealType) -> impl IntoView {
    let entry = entries.iter().find(|e| e.meal_type == meal);
    let color = match entry {
        Some(_) => "bg-primary", // Simplification: if exists, it's green (completed) or zinc-400 (planned)
        // In a real app we'd check a 'completed' field. For now let's assume if it exists it's green.
        None => "bg-zinc-200 dark:bg-zinc-700",
    };

    view! {
        <div class=format!("w-1.5 h-1.5 rounded-full {}", color)></div>
    }
}
