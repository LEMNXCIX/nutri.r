use crate::components::ui::{Loading, Modal};
use crate::tauri_bridge::{
    apply_recipe_edit, assign_plan_to_date, get_calendar_range, get_index, get_plan_detail,
    preview_recipe_edit, remove_calendar_entry, swap_calendar_entries, CalendarEntry, MealType,
    PlanDetail, RecipeSuggestion, StructuredPlan, StructuredRecipe,
};
use chrono::{Datelike, Duration, Local, NaiveDate};
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
struct AssignContext {
    meal_type: MealType,
    assignment_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct RecipePickerContext {
    meal_type: MealType,
    plan_id: String,
    assignment_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct RecipeCandidate {
    day_id: String,
    day_label: String,
    plan_day_index: u8,
    recipe: StructuredRecipe,
}

fn fallback_date() -> NaiveDate {
    Local::now().date_naive()
}

fn parse_route_date(value: &str) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap_or_else(|_| fallback_date())
}

fn weekday_label(date: NaiveDate) -> &'static str {
    match date.weekday() {
        chrono::Weekday::Mon => "Lunes",
        chrono::Weekday::Tue => "Martes",
        chrono::Weekday::Wed => "Miércoles",
        chrono::Weekday::Thu => "Jueves",
        chrono::Weekday::Fri => "Viernes",
        chrono::Weekday::Sat => "Sábado",
        chrono::Weekday::Sun => "Domingo",
    }
}

fn month_label(date: NaiveDate) -> &'static str {
    match date.month() {
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
        _ => "Mes",
    }
}

fn short_plan_id(plan_id: &str) -> String {
    plan_id.chars().take(8).collect::<String>()
}

fn meal_slots() -> [(MealType, &'static str, &'static str); 4] {
    [
        (MealType::Breakfast, "Desayuno", "07:30 AM"),
        (MealType::Lunch, "Almuerzo", "01:15 PM"),
        (MealType::Dinner, "Cena", "08:00 PM"),
        (MealType::Snack, "Snack", "04:30 PM"),
    ]
}

fn format_picker_meta(candidate: &RecipeCandidate) -> String {
    format!(
        "{} · {}",
        candidate.day_label,
        candidate.recipe.meal_type.display_name()
    )
}

fn format_human_date(date: &str) -> String {
    let parsed = parse_route_date(date);
    format!("{} {}", weekday_label(parsed), parsed.format("%d/%m"))
}

fn find_recipe<'a>(
    plan: &'a StructuredPlan,
    recipe_id: &str,
) -> Option<(&'a crate::tauri_bridge::StructuredDay, &'a StructuredRecipe)> {
    plan.days.iter().find_map(|day| {
        day.recipes
            .iter()
            .find(|recipe| recipe.recipe_id == recipe_id)
            .map(|recipe| (day, recipe))
    })
}

fn recipes_for_meal(detail: &PlanDetail, meal_type: MealType) -> Vec<RecipeCandidate> {
    detail
        .structured_plan
        .as_ref()
        .map(|plan| {
            plan.days
                .iter()
                .flat_map(|day| {
                    day.recipes
                        .iter()
                        .filter(move |recipe| recipe.meal_type == meal_type)
                        .cloned()
                        .map(move |recipe| RecipeCandidate {
                            day_id: day.day_id.clone(),
                            day_label: day.label.clone(),
                            plan_day_index: day.day_index,
                            recipe,
                        })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn resolve_entry_title(entry: &CalendarEntry, detail: Option<&PlanDetail>) -> String {
    if let (Some(recipe_id), Some(plan)) = (
        entry.recipe_id.as_deref(),
        detail.and_then(|item| item.structured_plan.as_ref()),
    ) {
        if let Some((_, recipe)) = find_recipe(plan, recipe_id) {
            return recipe.name.clone();
        }
    }

    format!("Plan {}", short_plan_id(&entry.plan_id))
}

fn resolve_entry_meta(entry: &CalendarEntry, detail: Option<&PlanDetail>) -> String {
    if let (Some(recipe_id), Some(plan)) = (
        entry.recipe_id.as_deref(),
        detail.and_then(|item| item.structured_plan.as_ref()),
    ) {
        if let Some((day, recipe)) = find_recipe(plan, recipe_id) {
            return format!("{} · {}", day.label, recipe.meal_type.display_name());
        }
    }

    if let Some(day_index) = entry.plan_day_index {
        return format!("Día {}", day_index + 1);
    }

    "Asignación manual".to_string()
}

fn resolve_entry_description(entry: &CalendarEntry, detail: Option<&PlanDetail>) -> String {
    if let (Some(recipe_id), Some(plan)) = (
        entry.recipe_id.as_deref(),
        detail.and_then(|item| item.structured_plan.as_ref()),
    ) {
        if let Some((_, recipe)) = find_recipe(plan, recipe_id) {
            if !recipe.ingredients.is_empty() {
                return recipe
                    .ingredients
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ");
            }

            if let Some(notes) = recipe.notes.as_ref().filter(|text| !text.trim().is_empty()) {
                return notes.clone();
            }
        }
    }

    format!("Plan {}", short_plan_id(&entry.plan_id))
}

fn swap_candidates(entries: &[CalendarEntry], current: &CalendarEntry) -> Vec<CalendarEntry> {
    entries
        .iter()
        .filter(|entry| {
            entry.date != current.date
                && entry.meal_type == current.meal_type
                && entry.plan_id == current.plan_id
                && entry.assignment_id == current.assignment_id
                && entry.recipe_id.is_some()
        })
        .cloned()
        .collect()
}

#[component]
pub fn DailyView() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let navigate_to_add = navigate.clone();
    let navigate_to_plan = navigate.clone();

    let date_param = move || {
        params
            .read()
            .get("date")
            .unwrap_or_else(|| fallback_date().format("%Y-%m-%d").to_string())
    };

    let parsed_date = move || parse_route_date(&date_param());

    let calendar_resource = LocalResource::new(move || {
        let date = date_param();
        async move {
            get_calendar_range(date.clone(), date)
                .await
                .unwrap_or_default()
        }
    });

    let surrounding_entries_resource = LocalResource::new(move || {
        let date = parsed_date();
        async move {
            get_calendar_range(
                (date - Duration::days(6)).to_string(),
                (date + Duration::days(6)).to_string(),
            )
            .await
            .unwrap_or_default()
        }
    });

    let plans_resource =
        LocalResource::new(move || async move { get_index().await.unwrap_or_default() });

    let (plan_details_cache, set_plan_details_cache) = signal(HashMap::<String, PlanDetail>::new());
    let (loading_plan_ids, set_loading_plan_ids) = signal(HashSet::<String>::new());

    let (assign_context, set_assign_context) = signal(Option::<AssignContext>::None);
    let (recipe_picker_context, set_recipe_picker_context) =
        signal(Option::<RecipePickerContext>::None);
    let (assign_feedback, set_assign_feedback) = signal(String::new());
    let (loading_plan_id, set_loading_plan_id) = signal(Option::<String>::None);
    let (assigning_recipe_id, set_assigning_recipe_id) = signal(Option::<String>::None);

    let (swap_context, set_swap_context) = signal(Option::<CalendarEntry>::None);
    let (swapping_date, set_swapping_date) = signal(Option::<String>::None);

    let (edit_context, set_edit_context) = signal(Option::<CalendarEntry>::None);
    let (edit_prompt, set_edit_prompt) = signal(String::new());
    let (edit_preview, set_edit_preview) = signal(Option::<RecipeSuggestion>::None);
    let (edit_feedback, set_edit_feedback) = signal(String::new());
    let (preview_loading, set_preview_loading) = signal(false);
    let (apply_loading, set_apply_loading) = signal(false);

    Effect::new(move |_| {
        let mut pending = HashSet::new();
        for entry in calendar_resource.get().unwrap_or_default() {
            pending.insert(entry.plan_id);
        }
        for entry in surrounding_entries_resource.get().unwrap_or_default() {
            pending.insert(entry.plan_id);
        }

        let cache = plan_details_cache.get();
        let loading = loading_plan_ids.get();
        for plan_id in pending {
            if cache.contains_key(&plan_id) || loading.contains(&plan_id) {
                continue;
            }

            set_loading_plan_ids.update(|ids| {
                ids.insert(plan_id.clone());
            });

            let plan_id_clone = plan_id.clone();
            spawn_local(async move {
                if let Ok(detail) = get_plan_detail(&plan_id_clone).await {
                    set_plan_details_cache.update(|items| {
                        items.insert(plan_id_clone.clone(), detail);
                    });
                }

                set_loading_plan_ids.update(|ids| {
                    ids.remove(&plan_id_clone);
                });
            });
        }
    });

    let on_remove = move |entry: CalendarEntry| {
        let calendar_resource = calendar_resource.clone();
        let surrounding_entries_resource = surrounding_entries_resource.clone();
        spawn_local(async move {
            if remove_calendar_entry(entry.date, entry.meal_type).await.is_ok() {
                calendar_resource.refetch();
                surrounding_entries_resource.refetch();
            }
        });
    };

    let open_plan_picker = move |meal_type: MealType, assignment_id: Option<String>| {
        set_assign_feedback.set(String::new());
        set_assign_context.set(Some(AssignContext {
            meal_type,
            assignment_id,
        }));
    };

    let open_recipe_picker_for_plan =
        move |plan_id: String, meal_type: MealType, assignment_id: Option<String>| {
            set_assign_feedback.set(String::new());

            if let Some(detail) = plan_details_cache.get().get(&plan_id).cloned() {
                if recipes_for_meal(&detail, meal_type).is_empty() {
                    set_assign_feedback.set(format!(
                        "Este plan no tiene recetas configuradas para {}.",
                        meal_type.display_name()
                    ));
                    return;
                }

                set_assign_context.set(None);
                set_recipe_picker_context.set(Some(RecipePickerContext {
                    meal_type,
                    plan_id,
                    assignment_id,
                }));
                return;
            }

            set_loading_plan_id.set(Some(plan_id.clone()));
            spawn_local(async move {
                match get_plan_detail(&plan_id).await {
                    Ok(detail) => {
                        let has_candidates = !recipes_for_meal(&detail, meal_type).is_empty();
                        set_plan_details_cache.update(|items| {
                            items.insert(plan_id.clone(), detail);
                        });

                        if has_candidates {
                            set_assign_context.set(None);
                            set_recipe_picker_context.set(Some(RecipePickerContext {
                                meal_type,
                                plan_id: plan_id.clone(),
                                assignment_id,
                            }));
                        } else {
                            set_assign_feedback.set(format!(
                                "Este plan no tiene recetas configuradas para {}.",
                                meal_type.display_name()
                            ));
                        }
                    }
                    Err(error) => {
                        set_assign_feedback.set(error);
                    }
                }

                set_loading_plan_id.set(None);
            });
        };

    let on_assign_recipe = move |context: RecipePickerContext, candidate: RecipeCandidate| {
        let date = date_param();
        let calendar_resource = calendar_resource.clone();
        let surrounding_entries_resource = surrounding_entries_resource.clone();
        set_assign_feedback.set(String::new());
        set_assigning_recipe_id.set(Some(candidate.recipe.recipe_id.clone()));
        spawn_local(async move {
            let result = assign_plan_to_date(
                date,
                context.meal_type,
                context.plan_id.clone(),
                Some(candidate.recipe.recipe_id.clone()),
                Some(candidate.plan_day_index),
                context.assignment_id.clone(),
            )
            .await;

            match result {
                Ok(_) => {
                    set_recipe_picker_context.set(None);
                    calendar_resource.refetch();
                    surrounding_entries_resource.refetch();
                }
                Err(error) => {
                    set_assign_feedback.set(error);
                }
            }

            set_assigning_recipe_id.set(None);
        });
    };

    let open_edit_modal = move |entry: CalendarEntry| {
        set_edit_feedback.set(String::new());
        set_edit_prompt.set(String::new());
        set_edit_preview.set(None);
        set_edit_context.set(Some(entry.clone()));

        if plan_details_cache.get().contains_key(&entry.plan_id) {
            return;
        }

        let plan_id = entry.plan_id.clone();
        spawn_local(async move {
            if let Ok(detail) = get_plan_detail(&plan_id).await {
                set_plan_details_cache.update(|items| {
                    items.insert(plan_id.clone(), detail);
                });
            }
        });
    };

    let on_preview_edit = move |_| {
        let Some(entry) = edit_context.get() else {
            return;
        };
        let Some(recipe_id) = entry.recipe_id.clone() else {
            return;
        };
        let prompt = edit_prompt.get().trim().to_string();
        if prompt.is_empty() {
            set_edit_feedback.set("Describe qué quieres cambiar en la receta.".to_string());
            return;
        }

        set_edit_feedback.set(String::new());
        set_preview_loading.set(true);
        spawn_local(async move {
            match preview_recipe_edit(&entry.plan_id, &recipe_id, prompt).await {
                Ok(preview) => {
                    set_edit_preview.set(Some(preview));
                }
                Err(error) => {
                    set_edit_feedback.set(error);
                }
            }

            set_preview_loading.set(false);
        });
    };

    let on_apply_edit = move |_| {
        let Some(entry) = edit_context.get() else {
            return;
        };
        let Some(preview) = edit_preview.get() else {
            return;
        };
        let Some(recipe_id) = entry.recipe_id.clone() else {
            return;
        };

        set_edit_feedback.set(String::new());
        set_apply_loading.set(true);
        let calendar_resource = calendar_resource.clone();
        let surrounding_entries_resource = surrounding_entries_resource.clone();

        spawn_local(async move {
            match apply_recipe_edit(&entry.plan_id, &recipe_id, preview.suggested_recipe.clone())
                .await
            {
                Ok(detail) => {
                    set_plan_details_cache.update(|items| {
                        items.insert(entry.plan_id.clone(), detail);
                    });
                    set_edit_context.set(None);
                    set_edit_prompt.set(String::new());
                    set_edit_preview.set(None);
                    calendar_resource.refetch();
                    surrounding_entries_resource.refetch();
                }
                Err(error) => {
                    set_edit_feedback.set(error);
                }
            }

            set_apply_loading.set(false);
        });
    };

    let open_swap_modal = move |entry: CalendarEntry| {
        set_swap_context.set(Some(entry.clone()));

        if plan_details_cache.get().contains_key(&entry.plan_id) {
            return;
        }

        let plan_id = entry.plan_id.clone();
        spawn_local(async move {
            if let Ok(detail) = get_plan_detail(&plan_id).await {
                set_plan_details_cache.update(|items| {
                    items.insert(plan_id.clone(), detail);
                });
            }
        });
    };

    let on_swap = move |current: CalendarEntry, target: CalendarEntry| {
        let calendar_resource = calendar_resource.clone();
        let surrounding_entries_resource = surrounding_entries_resource.clone();
        set_swapping_date.set(Some(target.date.clone()));

        spawn_local(async move {
            let result = swap_calendar_entries(
                current.date.clone(),
                current.meal_type,
                target.date.clone(),
                target.meal_type,
            )
            .await;

            if result.is_ok() {
                set_swap_context.set(None);
                calendar_resource.refetch();
                surrounding_entries_resource.refetch();
            }

            set_swapping_date.set(None);
        });
    };

    let _ = (
        plans_resource,
        assign_context,
        recipe_picker_context,
        assign_feedback,
        loading_plan_id,
        assigning_recipe_id,
        swap_context,
        swapping_date,
        edit_context,
        edit_prompt,
        edit_preview,
        edit_feedback,
        preview_loading,
        apply_loading,
        on_assign_recipe,
        on_preview_edit,
        on_apply_edit,
        on_swap,
    );

    view! {
        <div class="bg-white dark:bg-background-dark text-black dark:text-white min-h-screen">
            <header class="sticky top-0 z-50 bg-white dark:bg-background-dark border-b border-black dark:border-neutral-800 px-6 py-4 flex justify-between items-center">
                <button
                    on:click=move |_| {
                        if let Some(window) = web_sys::window() {
                            if let Ok(history) = window.history() {
                                let _ = history.back();
                            }
                        }
                    }
                    class="flex items-center justify-center"
                >
                    <span class="material-icons-outlined text-2xl">arrow_back</span>
                </button>
                <div class="text-[10px] tracking-[0.2em] font-black uppercase text-zinc-400">
                    Planning Mastery
                </div>
                <button
                    on:click=move |_| navigate("/calendar", Default::default())
                    class="flex items-center justify-center"
                >
                    <span class="material-icons-outlined text-2xl">calendar_month</span>
                </button>
            </header>

            <main class="px-6 pb-32">
                <section class="py-8 border-b-2 border-black dark:border-neutral-800">
                    <div class="flex flex-col">
                        <span class="text-sm font-black uppercase tracking-widest text-zinc-500 dark:text-zinc-400">
                            {move || weekday_label(parsed_date())}
                        </span>
                        <h1 class="text-7xl font-black uppercase leading-none mt-1">
                            {move || parsed_date().day()}
                        </h1>
                        <div class="mt-4 flex justify-between items-end gap-4">
                            <span class="text-xs font-bold uppercase tracking-tighter text-zinc-400">
                                {move || format!("{} {}", month_label(parsed_date()), parsed_date().year())}
                            </span>
                            <div class="flex gap-2">
                                <div class="px-3 py-1 bg-black text-white dark:bg-white dark:text-black text-[10px] font-black uppercase">
                                    Recetas
                                </div>
                                <div class="px-3 py-1 border border-black dark:border-neutral-700 text-[10px] font-black uppercase">
                                    Por día
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="grid grid-cols-3 gap-0 border-b border-black dark:border-neutral-800">
                    <div class="py-4 border-r border-black dark:border-neutral-800">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Comidas</div>
                        <div class="text-lg font-black">4</div>
                    </div>
                    <div class="py-4 px-4 border-r border-black dark:border-neutral-800">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Asignadas</div>
                        <div class="text-lg font-black">
                            {move || calendar_resource.get().unwrap_or_default().len()}
                        </div>
                    </div>
                    <div class="py-4 px-4">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Estado</div>
                        <div class="text-lg font-black uppercase">
                            {move || if calendar_resource.get().unwrap_or_default().is_empty() {
                                "Vacío".to_string()
                            } else {
                                "Activo".to_string()
                            }}
                        </div>
                    </div>
                </section>

                <div class="mt-8 space-y-12">
                    <Suspense fallback=move || view! { <div class="flex justify-center p-10"><Loading /></div> }>
                        {move || {
                            let entries = calendar_resource.get().unwrap_or_default();
                            let mut meal_map = HashMap::new();
                            for entry in entries {
                                meal_map.insert(entry.meal_type, entry);
                            }
                            let details = plan_details_cache.get();
                            let navigate_to_plan = navigate_to_plan.clone();

                            meal_slots()
                                .into_iter()
                                .map(|(meal_type, label, time)| {
                                    let entry = meal_map.get(&meal_type).cloned();
                                    let detail = entry
                                        .as_ref()
                                        .and_then(|item| details.get(&item.plan_id));
                                    let entry_for_class = entry.clone();

                                    let title = entry
                                        .as_ref()
                                        .map(|item| resolve_entry_title(item, detail))
                                        .unwrap_or_else(|| "No asignado".to_string());
                                    let meta = entry
                                        .as_ref()
                                        .map(|item| resolve_entry_meta(item, detail))
                                        .unwrap_or_else(|| "Sin receta para esta comida".to_string());
                                    let description = entry
                                        .as_ref()
                                        .map(|item| resolve_entry_description(item, detail))
                                        .unwrap_or_else(|| {
                                            "Elige un plan y una receta específica para este horario.".to_string()
                                        });

                                    let allow_recipe_change = entry
                                        .as_ref()
                                        .map(|item| {
                                            detail
                                                .map(|plan_detail| !recipes_for_meal(plan_detail, item.meal_type).is_empty())
                                                .unwrap_or(true)
                                        })
                                        .unwrap_or(false);

                                    let can_edit = entry
                                        .as_ref()
                                        .and_then(|item| item.recipe_id.as_ref())
                                        .is_some();
                                    let can_swap = entry
                                        .as_ref()
                                        .map(|item| item.assignment_id.is_some() && item.recipe_id.is_some())
                                        .unwrap_or(false);

                                    let detail_href = entry
                                        .as_ref()
                                        .map(|item| {
                                            item.recipe_id
                                                .as_ref()
                                                .map(|recipe_id| format!("/plan/{}/recipe/{}", item.plan_id, recipe_id))
                                                .unwrap_or_else(|| format!("/plan/{}", item.plan_id))
                                        })
                                        .unwrap_or_default();
                                    let navigate_to_plan_click = navigate_to_plan.clone();

                                    view! {
                                        <section>
                                            <div class="flex justify-between items-baseline mb-4">
                                                <h2 class="text-3xl font-black italic uppercase tracking-tighter">{label}</h2>
                                                <span class="text-xs font-bold text-zinc-400">{time}</span>
                                            </div>

                                            <div class="border border-zinc-200 dark:border-neutral-800 p-5 bg-white dark:bg-neutral-900">
                                                <div class="flex items-start gap-4">
                                                    <div class="w-16 h-16 bg-zinc-100 dark:bg-neutral-950 overflow-hidden border border-zinc-200 dark:border-neutral-800 flex-shrink-0 flex items-center justify-center">
                                                        <span class="material-icons-outlined text-zinc-400">restaurant</span>
                                                    </div>

                                                    <div class="flex-1 min-w-0">
                                                        <div class="flex items-start justify-between gap-3">
                                                            <div class="min-w-0">
                                                                <button
                                                                    on:click=move |_| {
                                                                        if !detail_href.is_empty() {
                                                                            navigate_to_plan_click(&detail_href, Default::default());
                                                                        }
                                                                    }
                                                                    class=move || {
                                                                        if entry_for_class.is_some() {
                                                                            "text-left"
                                                                        } else {
                                                                            "text-left cursor-default"
                                                                        }
                                                                    }
                                                                >
                                                                    <h3 class="font-black text-lg uppercase leading-tight break-words">
                                                                        {title.clone()}
                                                                    </h3>
                                                                </button>
                                                                <p class="mt-1 text-[10px] font-bold uppercase tracking-[0.2em] text-zinc-400">
                                                                    {meta.clone()}
                                                                </p>
                                                                <p class="mt-3 text-sm text-zinc-600 dark:text-zinc-300">
                                                                    {description.clone()}
                                                                </p>
                                                            </div>

                                                            {if entry.is_some() {
                                                                view! {
                                                                    <span class="material-icons-outlined text-[#00FF66] text-xl">check_circle</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <button
                                                                        on:click=move |_| open_plan_picker(meal_type, None)
                                                                        class="text-zinc-300 hover:text-primary transition-colors"
                                                                    >
                                                                        <span class="material-icons-outlined text-2xl">add_circle_outline</span>
                                                                    </button>
                                                                }.into_any()
                                                            }}
                                                        </div>

                                                        {if let Some(current_entry) = entry.clone() {
                                                            let current_for_swap = current_entry.clone();
                                                            let current_for_edit = current_entry.clone();
                                                            let current_for_remove = current_entry.clone();
                                                            let current_for_change = current_entry.clone();
                                                            let current_for_recipe = current_entry.clone();
                                                            let navigate_to_recipe = navigate_to_plan.clone();
                                                            view! {
                                                                <div class="mt-5 flex flex-wrap gap-2">
                                                                    <button
                                                                        on:click=move |_| {
                                                                            if allow_recipe_change {
                                                                                open_recipe_picker_for_plan(
                                                                                    current_for_change.plan_id.clone(),
                                                                                    current_for_change.meal_type,
                                                                                    current_for_change.assignment_id.clone(),
                                                                                );
                                                                            }
                                                                        }
                                                                        class="px-3 py-2 border border-zinc-200 dark:border-neutral-700 text-[10px] font-black uppercase tracking-[0.2em] hover:border-black dark:hover:border-white transition-colors"
                                                                    >
                                                                        Cambiar receta
                                                                    </button>

                                                                    {if can_swap {
                                                                        view! {
                                                                            <button
                                                                                on:click=move |_| open_swap_modal(current_for_swap.clone())
                                                                                class="px-3 py-2 border border-zinc-200 dark:border-neutral-700 text-[10px] font-black uppercase tracking-[0.2em] hover:border-black dark:hover:border-white transition-colors"
                                                                            >
                                                                                Intercambiar
                                                                            </button>
                                                                        }.into_any()
                                                                    } else {
                                                                        ().into_any()
                                                                    }}

                                                                    {if can_edit {
                                                                        view! {
                                                                            <button
                                                                                on:click=move |_| {
                                                                                    if let Some(recipe_id) = current_for_recipe.recipe_id.as_ref() {
                                                                                        navigate_to_recipe(
                                                                                            &format!("/plan/{}/recipe/{}", current_for_recipe.plan_id, recipe_id),
                                                                                            Default::default(),
                                                                                        );
                                                                                    }
                                                                                }
                                                                                class="px-3 py-2 border border-zinc-200 dark:border-neutral-700 text-[10px] font-black uppercase tracking-[0.2em] hover:border-black dark:hover:border-white transition-colors"
                                                                            >
                                                                                Ver preparación
                                                                            </button>
                                                                            <button
                                                                                on:click=move |_| open_edit_modal(current_for_edit.clone())
                                                                                class="px-3 py-2 border border-zinc-200 dark:border-neutral-700 text-[10px] font-black uppercase tracking-[0.2em] hover:border-black dark:hover:border-white transition-colors"
                                                                            >
                                                                                Editar con IA
                                                                            </button>
                                                                        }.into_any()
                                                                    } else {
                                                                        ().into_any()
                                                                    }}

                                                                    <button
                                                                        on:click=move |_| on_remove(current_for_remove.clone())
                                                                        class="px-3 py-2 border border-zinc-200 dark:border-neutral-700 text-[10px] font-black uppercase tracking-[0.2em] hover:border-red-500 hover:text-red-500 transition-colors"
                                                                    >
                                                                        Quitar
                                                                    </button>
                                                                </div>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <div class="mt-5">
                                                                    <button
                                                                        on:click=move |_| open_plan_picker(meal_type, None)
                                                                        class="px-4 py-3 bg-black dark:bg-white text-white dark:text-black text-[10px] font-black uppercase tracking-[0.3em]"
                                                                    >
                                                                        Asignar receta
                                                                    </button>
                                                                </div>
                                                            }.into_any()
                                                        }}
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="mt-6 h-[1px] bg-zinc-200 dark:bg-neutral-800"></div>
                                        </section>
                                    }
                                })
                                .collect_view()
                        }}
                    </Suspense>
                </div>

                <button
                    on:click=move |_| navigate_to_add("/add", Default::default())
                    class="mt-12 w-full bg-black dark:bg-white text-white dark:text-black py-5 font-black uppercase tracking-[0.3em] text-sm"
                >
                    Generar Nuevo Plan
                </button>
            </main>

            {move || if let Some(context) = assign_context.get() {
                let plans = plans_resource.get().unwrap_or_default();
                let cache = plan_details_cache.get();
                let plans_for_cards = StoredValue::new(plans.clone());
                let cache_for_cards = StoredValue::new(cache.clone());
                let context_for_cards = StoredValue::new(context.clone());
                view! {
                    <Portal>
                        <Modal on_close=Callback::new(move |_| {
                            set_assign_context.set(None);
                            set_assign_feedback.set(String::new());
                        })>
                            <div class="relative">
                                <div class="absolute top-0 left-0 w-full h-1 bg-accent"></div>
                                <div class="pt-4">
                                    <h3 class="text-2xl font-black uppercase tracking-tight text-neutral-950 dark:text-white">
                                        Asignar receta
                                    </h3>
                                    <p class="mt-2 text-[10px] font-bold uppercase tracking-[0.2em] text-neutral-400 dark:text-neutral-500">
                                        {format!("Selecciona un plan para {}", context.meal_type.display_name())}
                                    </p>

                                    {move || if !assign_feedback.get().is_empty() {
                                        view! { <p class="mt-4 text-sm text-red-500">{assign_feedback.get()}</p> }.into_any()
                                    } else {
                                        ().into_any()
                                    }}

                                    <div class="mt-8 max-h-[50vh] overflow-y-auto space-y-3 pr-1">
                                        {move || {
                                            plans_for_cards
                                                .get_value()
                                                .into_iter()
                                                .map(|plan| {
                                                    let plan_id = plan.id.clone();
                                                    let context_for_click = context_for_cards.get_value();
                                                    let cache_snapshot = cache_for_cards.get_value();
                                                    let title = cache_snapshot
                                                        .get(&plan_id)
                                                        .and_then(|detail| detail.structured_plan.as_ref().map(|item| item.title.clone()))
                                                        .unwrap_or_else(|| format!("Plan {}", short_plan_id(&plan_id)));
                                                    let proteins = if plan.proteinas.is_empty() {
                                                        "Sin proteínas registradas".to_string()
                                                    } else {
                                                        plan.proteinas.join(", ")
                                                    };
                                                    let plan_id_for_loading = plan_id.clone();
                                                    let plan_id_for_icon = plan_id.clone();
                                                    view! {
                                                        <button
                                                            on:click=move |_| open_recipe_picker_for_plan(
                                                                plan_id.clone(),
                                                                context_for_click.meal_type,
                                                                context_for_click.assignment_id.clone(),
                                                            )
                                                            disabled=move || loading_plan_id.get() == Some(plan_id_for_loading.clone())
                                                            class="w-full border border-neutral-200 dark:border-neutral-700 px-4 py-4 text-left hover:border-neutral-950 dark:hover:border-white hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-colors disabled:opacity-60"
                                                        >
                                                            <div class="flex items-center justify-between gap-4">
                                                                <div>
                                                                    <div class="text-xs font-black uppercase tracking-[0.2em] text-neutral-950 dark:text-white">
                                                                        {title}
                                                                    </div>
                                                                    <div class="mt-2 text-[10px] font-bold uppercase tracking-[0.15em] text-neutral-400 dark:text-neutral-500">
                                                                        {proteins}
                                                                    </div>
                                                                </div>
                                                                <span class="material-symbols-outlined text-neutral-400">
                                                                    {move || if loading_plan_id.get() == Some(plan_id_for_icon.clone()) {
                                                                        "sync"
                                                                    } else {
                                                                        "chevron_right"
                                                                    }}
                                                                </span>
                                                            </div>
                                                        </button>
                                                    }
                                                })
                                                .collect_view()
                                        }}
                                    </div>
                                </div>
                            </div>
                        </Modal>
                    </Portal>
                }.into_any()
            } else {
                ().into_any()
            }}

            {move || if let Some(context) = recipe_picker_context.get() {
                let detail = plan_details_cache.get().get(&context.plan_id).cloned();
                let candidates = detail
                    .as_ref()
                    .map(|item| recipes_for_meal(item, context.meal_type))
                    .unwrap_or_default();
                let has_candidates = !candidates.is_empty();
                let candidates_for_cards = StoredValue::new(candidates.clone());
                let context_for_cards = StoredValue::new(context.clone());
                view! {
                    <Portal>
                        <Modal on_close=Callback::new(move |_| {
                            set_recipe_picker_context.set(None);
                            set_assign_feedback.set(String::new());
                        })>
                            <div class="relative">
                                <div class="absolute top-0 left-0 w-full h-1 bg-accent"></div>
                                <div class="pt-4">
                                    <h3 class="text-2xl font-black uppercase tracking-tight text-neutral-950 dark:text-white">
                                        Elegir receta
                                    </h3>
                                    <p class="mt-2 text-[10px] font-bold uppercase tracking-[0.2em] text-neutral-400 dark:text-neutral-500">
                                        {format!("{} dentro del plan seleccionado", context.meal_type.display_name())}
                                    </p>

                                    {move || if !assign_feedback.get().is_empty() {
                                        view! { <p class="mt-4 text-sm text-red-500">{assign_feedback.get()}</p> }.into_any()
                                    } else {
                                        ().into_any()
                                    }}

                                    <div class="mt-8 max-h-[50vh] overflow-y-auto space-y-3 pr-1">
                                        {if !has_candidates {
                                            view! {
                                                <div class="border border-dashed border-neutral-200 dark:border-neutral-700 px-4 py-6 text-sm text-neutral-500">
                                                    Este plan no tiene recetas disponibles para esta comida.
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                {move || {
                                                    candidates_for_cards
                                                        .get_value()
                                                        .into_iter()
                                                        .map(|candidate| {
                                                            let context_for_click = context_for_cards.get_value();
                                                            let candidate_for_click = candidate.clone();
                                                            let recipe_id_for_loading = candidate.recipe.recipe_id.clone();
                                                            let recipe_id_for_icon = candidate.recipe.recipe_id.clone();
                                                            view! {
                                                                <button
                                                                    on:click=move |_| on_assign_recipe(context_for_click.clone(), candidate_for_click.clone())
                                                                    disabled=move || assigning_recipe_id.get() == Some(recipe_id_for_loading.clone())
                                                                    class="w-full border border-neutral-200 dark:border-neutral-700 px-4 py-4 text-left hover:border-neutral-950 dark:hover:border-white hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-colors disabled:opacity-60"
                                                                >
                                                                    <div class="flex items-start justify-between gap-4">
                                                                        <div>
                                                                            <div class="text-xs font-black uppercase tracking-[0.2em] text-neutral-950 dark:text-white">
                                                                                {candidate.recipe.name.clone()}
                                                                            </div>
                                                                            <div class="mt-2 text-[10px] font-bold uppercase tracking-[0.15em] text-neutral-400 dark:text-neutral-500">
                                                                                {format_picker_meta(&candidate)}
                                                                            </div>
                                                                            <div class="mt-3 text-sm text-neutral-600 dark:text-neutral-300">
                                                                                {candidate.recipe.ingredients.iter().take(4).cloned().collect::<Vec<_>>().join(", ")}
                                                                            </div>
                                                                        </div>
                                                                        <span class="material-symbols-outlined text-neutral-400">
                                                                            {move || if assigning_recipe_id.get() == Some(recipe_id_for_icon.clone()) {
                                                                                "sync"
                                                                            } else {
                                                                                "add"
                                                                            }}
                                                                        </span>
                                                                    </div>
                                                                </button>
                                                            }
                                                        })
                                                        .collect_view()
                                                }}
                                            }.into_any()
                                        }}
                                    </div>
                                </div>
                            </div>
                        </Modal>
                    </Portal>
                }.into_any()
            } else {
                ().into_any()
            }}

            {move || if let Some(current) = swap_context.get() {
                let candidates = swap_candidates(
                    &surrounding_entries_resource.get().unwrap_or_default(),
                    &current,
                );
                let plan_detail = StoredValue::new(
                    plan_details_cache.get().get(&current.plan_id).cloned(),
                );
                let has_candidates = !candidates.is_empty();
                let current_for_cards = StoredValue::new(current.clone());
                let candidates_for_cards = StoredValue::new(candidates.clone());
                view! {
                    <Portal>
                        <Modal on_close=Callback::new(move |_| set_swap_context.set(None))>
                            <div class="relative">
                                <div class="absolute top-0 left-0 w-full h-1 bg-accent"></div>
                                <div class="pt-4">
                                    <h3 class="text-2xl font-black uppercase tracking-tight text-neutral-950 dark:text-white">
                                        Intercambiar receta
                                    </h3>
                                    <p class="mt-2 text-[10px] font-bold uppercase tracking-[0.2em] text-neutral-400 dark:text-neutral-500">
                                        Solo se muestran días de la misma asignación semanal
                                    </p>

                                    <div class="mt-8 max-h-[50vh] overflow-y-auto space-y-3 pr-1">
                                        {if !has_candidates {
                                            view! {
                                                <div class="border border-dashed border-neutral-200 dark:border-neutral-700 px-4 py-6 text-sm text-neutral-500">
                                                    No hay otra receta compatible para intercambiar dentro de esta semana.
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                {move || {
                                                    candidates_for_cards
                                                        .get_value()
                                                        .into_iter()
                                                        .map(|candidate| {
                                                            let candidate_for_click = candidate.clone();
                                                            let current_for_click = current_for_cards.get_value();
                                                            let plan_detail_value = plan_detail.get_value();
                                                            let title = resolve_entry_title(&candidate, plan_detail_value.as_ref());
                                                            let meta = resolve_entry_meta(&candidate, plan_detail_value.as_ref());
                                                            let candidate_date_for_loading = candidate.date.clone();
                                                            let candidate_date_for_icon = candidate.date.clone();
                                                            view! {
                                                                <button
                                                                    on:click=move |_| on_swap(current_for_click.clone(), candidate_for_click.clone())
                                                                    disabled=move || swapping_date.get() == Some(candidate_date_for_loading.clone())
                                                                    class="w-full border border-neutral-200 dark:border-neutral-700 px-4 py-4 text-left hover:border-neutral-950 dark:hover:border-white hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-colors disabled:opacity-60"
                                                                >
                                                                    <div class="flex items-center justify-between gap-4">
                                                                        <div>
                                                                            <div class="text-xs font-black uppercase tracking-[0.2em] text-neutral-950 dark:text-white">
                                                                                {format_human_date(&candidate.date)}
                                                                            </div>
                                                                            <div class="mt-2 text-sm font-semibold text-neutral-700 dark:text-neutral-200">
                                                                                {title}
                                                                            </div>
                                                                            <div class="mt-1 text-[10px] font-bold uppercase tracking-[0.15em] text-neutral-400 dark:text-neutral-500">
                                                                                {meta}
                                                                            </div>
                                                                        </div>
                                                                        <span class="material-symbols-outlined text-neutral-400">
                                                                            {move || if swapping_date.get() == Some(candidate_date_for_icon.clone()) {
                                                                                "sync"
                                                                            } else {
                                                                                "swap_horiz"
                                                                            }}
                                                                        </span>
                                                                    </div>
                                                                </button>
                                                            }
                                                        })
                                                        .collect_view()
                                                }}
                                            }.into_any()
                                        }}
                                    </div>
                                </div>
                            </div>
                        </Modal>
                    </Portal>
                }.into_any()
            } else {
                ().into_any()
            }}

            {move || if let Some(current) = edit_context.get() {
                let detail = plan_details_cache.get().get(&current.plan_id).cloned();
                let original_recipe = current
                    .recipe_id
                    .as_deref()
                    .and_then(|recipe_id| {
                        detail
                            .as_ref()
                            .and_then(|item| item.structured_plan.as_ref())
                            .and_then(|plan| find_recipe(plan, recipe_id).map(|(_, recipe)| recipe.clone()))
                    });
                let preview = edit_preview.get();
                let original_recipe_for_view = StoredValue::new(original_recipe.clone());
                let preview_for_view = StoredValue::new(preview.clone());
                view! {
                    <Portal>
                        <Modal on_close=Callback::new(move |_| {
                            set_edit_context.set(None);
                            set_edit_prompt.set(String::new());
                            set_edit_preview.set(None);
                            set_edit_feedback.set(String::new());
                        })>
                            <div class="relative">
                                <div class="absolute top-0 left-0 w-full h-1 bg-accent"></div>
                                <div class="pt-4">
                                    <h3 class="text-2xl font-black uppercase tracking-tight text-neutral-950 dark:text-white">
                                        Editar receta con IA
                                    </h3>
                                    <p class="mt-2 text-[10px] font-bold uppercase tracking-[0.2em] text-neutral-400 dark:text-neutral-500">
                                        La actualización solo afecta esta receta del plan
                                    </p>

                                    {move || if !edit_feedback.get().is_empty() {
                                        view! { <p class="mt-4 text-sm text-red-500">{edit_feedback.get()}</p> }.into_any()
                                    } else {
                                        ().into_any()
                                    }}

                                    <div class="mt-6 space-y-4">
                                        {move || if let Some(recipe) = original_recipe_for_view.get_value() {
                                            view! {
                                                <div class="border border-neutral-200 dark:border-neutral-700 p-4">
                                                    <div class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-400">
                                                        Receta actual
                                                    </div>
                                                    <div class="mt-2 text-lg font-black uppercase">{recipe.name}</div>
                                                    <div class="mt-3 text-sm text-neutral-600 dark:text-neutral-300">
                                                        {recipe.ingredients.join(", ")}
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="border border-dashed border-neutral-200 dark:border-neutral-700 p-4 text-sm text-neutral-500">
                                                    No fue posible cargar la receta actual de este plan.
                                                </div>
                                            }.into_any()
                                        }}

                                        <textarea
                                            class="w-full min-h-28 border border-neutral-200 dark:border-neutral-700 bg-transparent px-4 py-3 text-sm outline-none"
                                            placeholder="Ejemplo: cambia el acompañamiento por una opción más ligera y agrega instrucciones más claras."
                                            on:input=move |ev| set_edit_prompt.set(event_target_value(&ev))
                                            prop:value=edit_prompt
                                        ></textarea>

                                        <button
                                            on:click=on_preview_edit
                                            disabled=move || preview_loading.get()
                                            class="w-full py-3 bg-black dark:bg-white text-white dark:text-black text-[10px] font-black uppercase tracking-[0.3em] disabled:opacity-60"
                                        >
                                            {move || if preview_loading.get() { "Generando sugerencia" } else { "Vista previa con IA" }}
                                        </button>

                                        {move || if let Some(suggestion) = preview_for_view.get_value() {
                                            let suggested = suggestion.suggested_recipe.clone();
                                            view! {
                                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                                    <div class="border border-neutral-200 dark:border-neutral-700 p-4">
                                                        <div class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-400">
                                                            Antes
                                                        </div>
                                                        <div class="mt-2 text-lg font-black uppercase">
                                                            {suggestion.original_recipe.name.clone()}
                                                        </div>
                                                        <div class="mt-3 text-sm text-neutral-600 dark:text-neutral-300">
                                                            {suggestion.original_recipe.ingredients.join(", ")}
                                                        </div>
                                                    </div>
                                                    <div class="border border-neutral-200 dark:border-neutral-700 p-4">
                                                        <div class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-400">
                                                            Después
                                                        </div>
                                                        <div class="mt-2 text-lg font-black uppercase">
                                                            {suggested.name.clone()}
                                                        </div>
                                                        <div class="mt-3 text-sm text-neutral-600 dark:text-neutral-300">
                                                            {suggested.ingredients.join(", ")}
                                                        </div>
                                                    </div>
                                                </div>
                                                <button
                                                    on:click=on_apply_edit
                                                    disabled=move || apply_loading.get()
                                                    class="w-full py-3 border border-black dark:border-white text-[10px] font-black uppercase tracking-[0.3em] disabled:opacity-60"
                                                >
                                                    {move || if apply_loading.get() { "Aplicando cambios" } else { "Confirmar actualización" }}
                                                </button>
                                            }.into_any()
                                        } else {
                                            ().into_any()
                                        }}
                                    </div>
                                </div>
                            </div>
                        </Modal>
                    </Portal>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
