use crate::components::ui::{Button, Card, Loading};
use crate::tauri_bridge::{
    assign_plan_to_date, get_calendar_range, get_index, remove_calendar_entry, CalendarEntry,
    MealType,
};
use chrono::{Datelike, Local, Month, NaiveDate};
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashMap;

#[component]
pub fn Calendar() -> impl IntoView {
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
        if let Some((date, meal)) = show_assign_modal.get() {
            let date_str = date.to_string();
            spawn_local(async move {
                if let Ok(_) = assign_plan_to_date(date_str, meal, plan_id).await {
                    calendar_resource.refetch();
                }
            });
            set_show_assign_modal.set(None);
        }
    };

    let on_remove = move |(date, meal): (String, MealType)| {
        spawn_local(async move {
            if let Ok(_) = remove_calendar_entry(date, meal).await {
                calendar_resource.refetch();
            }
        });
    };

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 animate-in fade-in duration-700">
            <header class="mb-10 flex flex-col md:flex-row md:items-end justify-between gap-6">
                <div>
                    <span class="inline-block px-4 py-1.5 rounded-full bg-green-500/10 text-green-400 text-[10px] font-black uppercase tracking-[0.2em] mb-4">
                        "Meal Planning"
                    </span>
                    <h2 class="text-4xl font-black text-white tracking-tighter mb-2 leading-none uppercase">
                        {move || format!("{} {}", Month::try_from(current_month.get() as u8).ok().map(|m| m.name()).unwrap_or(""), current_year.get())}
                    </h2>
                    <div class="h-1.5 w-12 bg-green-500 rounded-full mb-4"></div>
                    <p class="text-gray-400 font-medium font-black uppercase tracking-[0.1em] text-[10px]">"Organiza tus planes nutricionales por día."</p>
                </div>

                <div class="flex items-center gap-2 glass rounded-2xl p-1.5 border-white/5 shadow-2xl">
                    <button on:click=on_prev_month class="p-3 hover:bg-white/5 rounded-xl transition-all text-gray-400 hover:text-white active:scale-90">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                    </button>
                    <div class="w-px h-6 bg-white/5 mx-1"></div>
                    <button on:click=on_next_month class="p-3 hover:bg-white/5 rounded-xl transition-all text-gray-400 hover:text-white active:scale-90">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                    </button>
                </div>
            </header>

            <div class="glass rounded-[2.5rem] overflow-hidden border-white/5 shadow-2xl">
                <div class="grid grid-cols-7 bg-white/5">
                    {vec!["LUN", "MAR", "MIÉ", "JUE", "VIE", "SÁB", "DOM"].into_iter().map(|day| view! {
                        <div class="p-4 text-center text-[10px] font-black text-gray-400 uppercase tracking-[0.2em] border-b border-white/5">
                            {day}
                        </div>
                    }).collect::<Vec<_>>()}
                </div>

                <div class="grid grid-cols-7 gap-px bg-white/5">
                    <Suspense fallback=move || view! { <div class="col-span-7 p-24 flex justify-center bg-gray-950/50"><Loading /></div> }>
                        {move || {
                            let entries = calendar_resource.get().unwrap_or_default();
                            let mut entries_map = HashMap::new();
                            for e in entries {
                                entries_map.entry(e.date.clone()).or_insert_with(Vec::new).push(e);
                            }

                            let year = current_year.get();
                            let month = current_month.get();
                            let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                            let weekday = first_day.weekday().number_from_monday(); // 1-7

                            let days_in_month = if month == 12 {
                                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().pred_opt().unwrap().day()
                            } else {
                                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().pred_opt().unwrap().day()
                            };

                            let mut grid_items = Vec::new();

                            // Padding from prev month
                            for _ in 1..weekday {
                                grid_items.push(view! { <div class="bg-gray-950/20 min-h-[160px]"></div> }.into_any());
                            }

                            // Days of current month
                            for d in 1..=days_in_month {
                                let date = NaiveDate::from_ymd_opt(year, month, d).unwrap();
                                let date_str = date.to_string();
                                let day_entries = entries_map.get(&date_str).cloned().unwrap_or_default();
                                let is_today = date == now;

                                let date_for_render_b = date.clone();
                                let date_for_render_l = date.clone();
                                let date_for_render_d = date.clone();

                                let on_remove_b = on_remove.clone();
                                let on_remove_l = on_remove.clone();
                                let on_remove_d = on_remove.clone();

                                grid_items.push(view! {
                                    <div class=format!("bg-gray-950/40 min-h-[160px] p-3 flex flex-col gap-2 group transition-all hover:bg-white/5 {}", if is_today { "bg-green-500/5 ring-1 ring-inset ring-green-500/30" } else { "" })>
                                        <span class=format!("text-xs font-black w-8 h-8 flex items-center justify-center rounded-xl transition-all {}", if is_today { "bg-green-500 text-gray-950 shadow-lg shadow-green-500/20" } else { "text-gray-500 group-hover:text-white" })>
                                            {d}
                                        </span>

                                        <div class="flex flex-col gap-1.5 flex-1">
                                            {render_meal_assigned(&day_entries, MealType::Breakfast, date_for_render_b, set_show_assign_modal, Callback::new(on_remove_b))}
                                            {render_meal_assigned(&day_entries, MealType::Lunch, date_for_render_l, set_show_assign_modal, Callback::new(on_remove_l))}
                                            {render_meal_assigned(&day_entries, MealType::Dinner, date_for_render_d, set_show_assign_modal, Callback::new(on_remove_d))}
                                        </div>
                                    </div>
                                }.into_any());
                            }

                            grid_items
                        }}
                    </Suspense>
                </div>
            </div>

            // Assign Modal
            {move || if let Some((date, meal)) = show_assign_modal.get() {
                view! {
                    <div class="fixed inset-0 bg-black/90 backdrop-blur-xl z-[200] flex items-center justify-center p-4 animate-in fade-in duration-300">
                        <Card class="max-w-md w-full p-10 glass rounded-[3rem] border-white/10 shadow-3xl relative overflow-hidden">
                            <div class="absolute -top-12 -right-12 w-32 h-32 bg-green-500/10 blur-3xl"></div>

                            <h3 class="text-2xl font-black text-white mb-2 tracking-tighter uppercase italic">"Asignar Plan"</h3>
                            <p class="text-gray-400 mb-8 text-[11px] font-black uppercase tracking-widest leading-relaxed">
                                {format!("Elige un plan para el {} de {} el {}.",
                                    match meal {
                                        MealType::Breakfast => "desayuno",
                                        MealType::Lunch => "almuerzo",
                                        MealType::Dinner => "cena",
                                        MealType::Snack => "merienda",
                                    },
                                    Month::try_from(date.month() as u8).ok().map(|m| m.name()).unwrap_or(""),
                                    date.day())}
                            </p>

                            <div class="max-h-[350px] overflow-y-auto space-y-3 mb-8 pr-4 custom-scrollbar">
                                {move || plans.get().map(|list| {
                                    if list.is_empty() {
                                        return view! { <div class="text-center py-12 text-gray-600 font-bold uppercase tracking-widest text-[10px]">"No tienes planes guardados"</div> }.into_any()
                                    }
                                    list.into_iter().map(|plan| {
                                        let pid = plan.id.clone();
                                        view! {
                                            <button
                                                on:click=move |_| on_assign(pid.clone())
                                                class="w-full text-left p-4 rounded-2xl glass-light hover:bg-green-500/10 border-white/5 hover:border-green-500/30 transition-all group active:scale-95"
                                            >
                                                <div class="font-black text-gray-200 group-hover:text-green-400 text-xs tracking-widest uppercase mb-1">{plan.id.chars().take(12).collect::<String>()}</div>
                                                <div class="text-[10px] text-gray-500 font-black uppercase tracking-tighter">{plan.fecha.clone()}</div>
                                            </button>
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                }).unwrap_or_else(|| ().into_any())}
                            </div>

                            <div class="flex flex-col gap-3">
                                <Button
                                    on_click=Callback::new(move |_| set_show_assign_modal.set(None))
                                    class="w-full bg-white/5 hover:bg-white/10 text-white border-white/5".to_string()
                                >
                                    "Cerrar"
                                </Button>
                            </div>
                        </Card>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}

fn render_meal_assigned(
    entries: &[CalendarEntry],
    meal: MealType,
    date: NaiveDate,
    set_modal: WriteSignal<Option<(NaiveDate, MealType)>>,
    on_remove: Callback<(String, MealType)>,
) -> impl IntoView {
    let entry = entries.iter().find(|e| e.meal_type == meal);
    let (label, color_class, bg_class) = match meal {
        MealType::Breakfast => ("D", "text-yellow-500", "bg-yellow-500/10"),
        MealType::Lunch => ("A", "text-blue-500", "bg-blue-500/10"),
        MealType::Dinner => ("C", "text-purple-500", "bg-purple-500/10"),
        _ => ("S", "text-orange-500", "bg-orange-500/10"),
    };

    let date_str = date.to_string();

    view! {
        <div class="flex items-center gap-1 group/meal min-h-[30px]">
            <span class=format!("text-[10px] font-black w-5 h-5 flex items-center justify-center rounded-lg uppercase tracking-tighter shrink-0 {} {}", color_class, bg_class)>
                {label}
            </span>

            {if let Some(e) = entry {
                let pid = e.plan_id.clone();
                let m = meal.clone();
                let d = date_str.clone();
                view! {
                    <div class="flex-1 flex items-center justify-between glass-light rounded-lg px-2 py-0.5 border-white/5 hover:bg-white/10 transition-all truncate">
                        <span class="text-[9px] text-gray-300 font-black uppercase tracking-tighter truncate shrink">{pid.chars().take(8).collect::<String>()}</span>
                        <button
                            on:click=move |ev| {
                                ev.stop_propagation();
                                on_remove.run((d.clone(), m.clone()));
                            }
                            class="opacity-0 group-hover/meal:opacity-100 p-0.5 hover:text-red-500 transition-all"
                        >
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                }.into_any()
            } else {
                let m = meal.clone();
                view! {
                    <button
                        on:click=move |_| set_modal.set(Some((date, m.clone())))
                        class="flex-1 flex items-center gap-1 text-[9px] text-gray-600 font-black uppercase tracking-[0.1em] hover:text-green-500 transition-colors opacity-0 group-hover:opacity-100"
                    >
                        <svg class="w-2.5 h-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                        </svg>
                        "Asignar"
                    </button>
                }.into_any()
            }}
        </div>
    }
}
