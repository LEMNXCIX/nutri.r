use crate::components::ui::{Button, Card, Loading};
use crate::tauri_bridge::{
    assign_plan_to_date, get_calendar_range, get_index, remove_calendar_entry, CalendarEntry,
    MealType,
};
use chrono::{Datelike, Local, Month, NaiveDate};
use leptos::portal::Portal;
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
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 animate-in fade-in duration-700 font-sans text-gray-900">
            <header class="mb-10 flex flex-col md:flex-row md:items-end justify-between gap-6">
                <div>
                    <span class="inline-block px-4 py-1.5 rounded-full border border-gray-200 text-gray-500 text-[10px] font-black uppercase tracking-[0.2em] mb-4">
                        "Meal Planning"
                    </span>
                    <h2 class="text-4xl font-black text-black tracking-tighter mb-2 leading-none uppercase">
                        {move || format!("{} {}", Month::try_from(current_month.get() as u8).ok().map(|m| m.name()).unwrap_or(""), current_year.get())}
                    </h2>
                    <div class="h-1.5 w-12 bg-black rounded-full mb-4"></div>
                    <p class="text-gray-500 font-medium font-black uppercase tracking-[0.1em] text-[10px]">"Organiza tus planes nutricionales por día."</p>
                </div>

                <div class="flex items-center gap-2 bg-white rounded-2xl p-1.5 border border-gray-200 shadow-sm">
                    <button on:click=on_prev_month class="p-3 hover:bg-gray-50 rounded-xl transition-all text-gray-400 hover:text-black active:scale-90">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                    </button>
                    <div class="w-px h-6 bg-gray-100 mx-1"></div>
                    <button on:click=on_next_month class="p-3 hover:bg-gray-50 rounded-xl transition-all text-gray-400 hover:text-black active:scale-90">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                    </button>
                </div>
            </header>

            // Desktop View (7-column Grid)
            <div class="hidden md:block bg-white rounded-[2.5rem] overflow-hidden border border-gray-200 shadow-xl">
                <div class="grid grid-cols-7 bg-gray-50 border-b border-gray-200">
                    {vec!["LUN", "MAR", "MIÉ", "JUE", "VIE", "SÁB", "DOM"].into_iter().map(|day| view! {
                        <div class="p-4 text-center text-[10px] font-black text-gray-400 uppercase tracking-[0.2em] border-r border-gray-100 last:border-r-0">
                            {day}
                        </div>
                    }).collect::<Vec<_>>()}
                </div>

                <div class="grid grid-cols-7 gap-px bg-gray-200">
                    <Suspense fallback=move || view! { <div class="col-span-7 p-24 flex justify-center bg-gray-50"><Loading /></div> }>
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
                                grid_items.push(view! { <div class="bg-gray-50/50 min-h-[160px]"></div> }.into_any());
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
                                    <div class=format!("bg-white min-h-[160px] p-3 flex flex-col gap-2 group transition-all hover:bg-gray-50 {}", if is_today { "ring-nuni ring-inset ring-2 ring-black" } else { "" })>
                                        <span class=format!("text-xs font-black w-8 h-8 flex items-center justify-center rounded-xl transition-all {}", if is_today { "bg-black text-white shadow-lg" } else { "text-gray-400 group-hover:text-black" })>
                                            {d}
                                        </span>

                                        <div class="flex flex-col gap-1.5 flex-1">
                                            {render_meal_assigned(&day_entries, MealType::Breakfast, date_for_render_b, set_show_assign_modal, Callback::new(on_remove_b), false)}
                                            {render_meal_assigned(&day_entries, MealType::Lunch, date_for_render_l, set_show_assign_modal, Callback::new(on_remove_l), false)}
                                            {render_meal_assigned(&day_entries, MealType::Dinner, date_for_render_d, set_show_assign_modal, Callback::new(on_remove_d), false)}
                                        </div>
                                    </div>
                                }.into_any());
                            }

                            grid_items
                        }}
                    </Suspense>
                </div>
            </div>

            // Mobile View (Vertical List)
            <div class="md:hidden space-y-4">
                <Suspense fallback=move || view! { <div class="py-20 flex justify-center"><Loading /></div> }>
                    {move || {
                        let entries = calendar_resource.get().unwrap_or_default();
                        let mut entries_map = HashMap::new();
                        for e in entries {
                            entries_map.entry(e.date.clone()).or_insert_with(Vec::new).push(e);
                        }

                        let year = current_year.get();
                        let month = current_month.get();
                        let days_in_month = if month == 12 {
                            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().pred_opt().unwrap().day()
                        } else {
                            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().pred_opt().unwrap().day()
                        };

                        (1..=days_in_month).map(|d| {
                            let date = NaiveDate::from_ymd_opt(year, month, d).unwrap();
                            let date_str = date.to_string();
                            let day_entries = entries_map.get(&date_str).cloned().unwrap_or_default();
                            let is_today = date == now;
                            let day_name = match date.weekday() {
                                chrono::Weekday::Mon => "Lunes",
                                chrono::Weekday::Tue => "Martes",
                                chrono::Weekday::Wed => "Miércoles",
                                chrono::Weekday::Thu => "Jueves",
                                chrono::Weekday::Fri => "Viernes",
                                chrono::Weekday::Sat => "Sábado",
                                chrono::Weekday::Sun => "Domingo",
                            };

                            let date_for_render_b = date.clone();
                            let date_for_render_l = date.clone();
                            let date_for_render_d = date.clone();

                            let on_remove_b = on_remove.clone();
                            let on_remove_l = on_remove.clone();
                            let on_remove_d = on_remove.clone();

                            view! {
                                <Card class=format!("p-5 bg-white rounded-3xl border border-gray-100 shadow-sm {}", if is_today { "ring-2 ring-black" } else { "" })>
                                    <div class="flex items-center justify-between mb-4">
                                        <div class="flex items-center gap-3">
                                            <span class=format!("text-lg font-black w-10 h-10 flex items-center justify-center rounded-2xl {}", if is_today { "bg-black text-white" } else { "bg-gray-100 text-black" })>
                                                {d}
                                            </span>
                                            <div class="flex flex-col">
                                                <span class="text-xs font-black uppercase tracking-widest text-black">{day_name}</span>
                                                {if is_today {
                                                    Some(view! { <span class="text-[9px] font-black text-gray-400 uppercase tracking-tighter">"HOY"</span> })
                                                } else {
                                                    None
                                                }}
                                            </div>
                                        </div>
                                    </div>

                                    <div class="space-y-3">
                                        {render_meal_assigned(&day_entries, MealType::Breakfast, date_for_render_b, set_show_assign_modal, Callback::new(on_remove_b), true)}
                                        {render_meal_assigned(&day_entries, MealType::Lunch, date_for_render_l, set_show_assign_modal, Callback::new(on_remove_l), true)}
                                        {render_meal_assigned(&day_entries, MealType::Dinner, date_for_render_d, set_show_assign_modal, Callback::new(on_remove_d), true)}
                                    </div>
                                </Card>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </Suspense>
            </div>

            // Assign Modal
            {move || if let Some((date, meal)) = show_assign_modal.get() {
                view! {
                    <Portal>
                        <div class="fixed inset-0 bg-white/90 backdrop-blur-xl z-[200] flex items-center justify-center p-4 animate-in fade-in duration-300">
                            <Card class="max-w-md w-full p-6 md:p-10 bg-white rounded-3xl md:rounded-[3rem] border border-gray-200 shadow-2xl relative overflow-hidden text-center">

                                <h3 class="text-2xl font-black text-black mb-2 tracking-tighter uppercase italic">"Asignar Plan"</h3>
                                <p class="text-gray-500 mb-8 text-[11px] font-black uppercase tracking-widest leading-relaxed">
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
                                            return view! { <div class="text-center py-12 text-gray-400 font-bold uppercase tracking-widest text-[10px]">"No tienes planes guardados"</div> }.into_any()
                                        }
                                        list.into_iter().map(|plan| {
                                            let pid = plan.id.clone();
                                            view! {
                                                <button
                                                    on:click=move |_| on_assign(pid.clone())
                                                    class="w-full text-left p-4 rounded-2xl bg-gray-50 hover:bg-black hover:text-white border border-gray-100 transition-all group active:scale-95"
                                                >
                                                    <div class="font-black text-gray-900 group-hover:text-white text-xs tracking-widest uppercase mb-1">{plan.id.chars().take(12).collect::<String>()}</div>
                                                    <div class="text-[10px] text-gray-500 group-hover:text-gray-300 font-black uppercase tracking-tighter">{plan.fecha.clone()}</div>
                                                </button>
                                            }
                                        }).collect::<Vec<_>>().into_any()
                                    }).unwrap_or_else(|| ().into_any())}
                                </div>

                                <div class="flex flex-col gap-3">
                                    <Button
                                        on_click=Callback::new(move |_| set_show_assign_modal.set(None))
                                        class="w-full bg-white hover:bg-gray-50 text-gray-900 border border-gray-200".to_string()
                                    >
                                        "Cerrar"
                                    </Button>
                                </div>
                            </Card>
                        </div>
                    </Portal>
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
    is_mobile: bool,
) -> impl IntoView {
    let entry = entries.iter().find(|e| e.meal_type == meal);

    // Labels and colors for mobile vs desktop
    let (label_full, label_short, color_class, bg_class) = match meal {
        MealType::Breakfast => ("Desayuno", "D", "text-yellow-600", "bg-yellow-50"),
        MealType::Lunch => ("Almuerzo", "A", "text-blue-600", "bg-blue-50"),
        MealType::Dinner => ("Cena", "C", "text-purple-600", "bg-purple-50"),
        _ => ("Merienda", "S", "text-orange-600", "bg-orange-50"),
    };

    let date_str = date.to_string();

    view! {
        <div class="flex items-center gap-2 group/meal min-h-[40px]">
            {if is_mobile {
                view! {
                    <span class=format!("text-[9px] font-black px-2 py-1 rounded-lg uppercase tracking-widest shrink-0 {} {}", color_class, bg_class)>
                        {label_full}
                    </span>
                }.into_any()
            } else {
                view! {
                    <span class=format!("text-[10px] font-black w-5 h-5 flex items-center justify-center rounded-lg uppercase tracking-tighter shrink-0 {} {}",
                        color_class.replace("-600", "-500"),
                        bg_class.replace("bg-", "bg-").replace("-50", "-500/10")
                    )>
                        {label_short}
                    </span>
                 }.into_any()
            }}

            <div class="flex-1">
                {if let Some(e) = entry {
                    let pid = e.plan_id.clone();
                    let m = meal.clone();
                    let d = date_str.clone();
                    view! {
                        <div class="flex items-center justify-between bg-gray-50 rounded-xl px-3 py-2 border border-gray-100 hover:border-black transition-all group/item">
                            <span class="text-[10px] text-black font-black uppercase tracking-tighter truncate shrink">
                                {pid.chars().take(if is_mobile { 20 } else { 8 }).collect::<String>()}
                            </span>
                            <button
                                on:click=move |ev| {
                                    ev.stop_propagation();
                                    on_remove.run((d.clone(), m.clone()));
                                }
                                class=format!("hover:text-red-500 transition-all {}", if is_mobile { "opacity-100 p-1" } else { "opacity-0 group-hover/meal:opacity-100 p-0.5" })
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
                            class=format!("w-full flex items-center gap-2 text-[10px] text-gray-400 font-black uppercase tracking-widest hover:text-black transition-colors py-2 px-3 border border-dashed border-gray-200 rounded-xl {}",
                                if is_mobile { "opacity-100" } else { "opacity-0 group-hover:opacity-100" })
                        >
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                            </svg>
                            "Asignar"
                        </button>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
