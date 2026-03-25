use crate::components::ui::{Loading, Modal};
use crate::tauri_bridge::{
    apply_recipe_edit, assign_weekly_plan_to_date, get_plan_detail, preview_recipe_edit,
    RecipeSuggestion, StructuredDay, StructuredPlan, StructuredRecipe,
};
use chrono::{Datelike, Duration, Local, Weekday};
use leptos::logging::log;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use web_sys::Storage;

#[derive(Clone, Debug)]
struct RecipeDetailData {
    plan_id: String,
    plan_title: String,
    plan_instructions: Option<String>,
    day: StructuredDay,
    recipe: StructuredRecipe,
}

fn find_recipe(plan: &StructuredPlan, recipe_id: &str) -> Option<(StructuredDay, StructuredRecipe)> {
    plan.days.iter().find_map(|day| {
        day.recipes
            .iter()
            .find(|recipe| recipe.recipe_id == recipe_id)
            .map(|recipe| (day.clone(), recipe.clone()))
    })
}

fn get_next_weekday(weekday: Weekday) -> String {
    let now = Local::now().date_naive();
    let mut days_ahead =
        weekday.number_from_monday() as i32 - now.weekday().number_from_monday() as i32;
    if days_ahead <= 0 {
        days_ahead += 7;
    }
    (now + Duration::days(days_ahead as i64))
        .format("%Y-%m-%d")
        .to_string()
}

fn checklist_storage_key(plan_id: &str, recipe_id: &str) -> String {
    format!("recipe-checklist:{}:{}", plan_id, recipe_id)
}

fn get_local_storage() -> Option<Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
}

fn load_checked_ingredients(storage_key: &str, ingredients: &[String]) -> Vec<String> {
    let Some(storage) = get_local_storage() else {
        return Vec::new();
    };

    let Ok(Some(raw_value)) = storage.get_item(storage_key) else {
        return Vec::new();
    };

    serde_json::from_str::<Vec<String>>(&raw_value)
        .unwrap_or_default()
        .into_iter()
        .filter(|item| ingredients.contains(item))
        .collect()
}

fn save_checked_ingredients(storage_key: &str, checked_ingredients: &[String]) {
    let Some(storage) = get_local_storage() else {
        return;
    };

    if checked_ingredients.is_empty() {
        let _ = storage.remove_item(storage_key);
        return;
    }

    if let Ok(serialized) = serde_json::to_string(checked_ingredients) {
        let _ = storage.set_item(storage_key, &serialized);
    }
}

#[component]
pub fn RecipeDetail() -> impl IntoView {
    let params = use_params_map();
    let plan_id = move || params.with(|items| items.get("id").unwrap_or_default());
    let recipe_id = move || params.with(|items| items.get("recipe_id").unwrap_or_default());
    let (refresh_nonce, set_refresh_nonce) = signal(0_u32);
    let (show_assign_modal, set_show_assign_modal) = signal(false);
    let (assigning_weekly, set_assigning_weekly) = signal(false);
    let (show_edit_modal, set_show_edit_modal) = signal(false);
    let (edit_prompt, set_edit_prompt) = signal(String::new());
    let (edit_preview, set_edit_preview) = signal(Option::<RecipeSuggestion>::None);
    let (edit_feedback, set_edit_feedback) = signal(String::new());
    let (preview_loading, set_preview_loading) = signal(false);
    let (apply_loading, set_apply_loading) = signal(false);
    let (checked_ingredients, set_checked_ingredients) = signal(Vec::<String>::new());

    let on_assign_weekly = Callback::new(move |start_date: String| {
        let plan_id_value = plan_id();
        set_assigning_weekly.set(true);
        spawn_local(async move {
            if let Err(error) = assign_weekly_plan_to_date(&start_date, &plan_id_value).await {
                log!("Error assigning weekly plan from recipe detail: {}", error);
            } else {
                set_show_assign_modal.set(false);
            }
            set_assigning_weekly.set(false);
        });
    });

    let recipe_resource = LocalResource::new(move || {
        let plan_id_value = plan_id();
        let recipe_id_value = recipe_id();
        let _refresh = refresh_nonce.get();
        async move {
            match get_plan_detail(&plan_id_value).await {
                Ok(detail) => {
                    let Some(plan) = detail.structured_plan else {
                        return Err("Este plan no tiene detalle estructurado para mostrar la receta.".to_string());
                    };

                    let plan_title = if plan.title.trim().is_empty() {
                        format!("Plan {}", plan_id_value.chars().take(8).collect::<String>())
                    } else {
                        plan.title.clone()
                    };

                    match find_recipe(&plan, &recipe_id_value) {
                        Some((day, recipe)) => Ok(RecipeDetailData {
                            plan_id: plan_id_value,
                            plan_title,
                            plan_instructions: plan.instructions.clone(),
                            day,
                            recipe,
                        }),
                        None => Err("No encontramos esa receta dentro del plan seleccionado.".to_string()),
                    }
                }
                Err(error) => Err(error),
            }
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(data)) = recipe_resource.get() {
            let storage_key = checklist_storage_key(&data.plan_id, &data.recipe.recipe_id);
            let stored_items = load_checked_ingredients(&storage_key, &data.recipe.ingredients);
            set_checked_ingredients.set(stored_items);
        }
    });

    Effect::new(move |_| {
        let Some(Ok(data)) = recipe_resource.get() else {
            return;
        };

        let storage_key = checklist_storage_key(&data.plan_id, &data.recipe.recipe_id);
        let checked_items = checked_ingredients.get();
        save_checked_ingredients(&storage_key, &checked_items);
    });

    let toggle_ingredient = Callback::new(move |ingredient: String| {
        set_checked_ingredients.update(|items| {
            if let Some(index) = items.iter().position(|item| item == &ingredient) {
                items.remove(index);
            } else {
                items.push(ingredient);
            }
        });
    });

    let open_edit_modal = move |_| {
        set_edit_prompt.set(String::new());
        set_edit_preview.set(None);
        set_edit_feedback.set(String::new());
        set_show_edit_modal.set(true);
    };

    let on_preview_edit = Callback::new(move |_| {
        let Some(Some(Ok(data))) = recipe_resource.try_get() else {
            return;
        };

        let prompt = edit_prompt.get().trim().to_string();
        if prompt.is_empty() {
            set_edit_feedback.set("Describe qué quieres cambiar en la receta.".to_string());
            return;
        }

        set_edit_feedback.set(String::new());
        set_preview_loading.set(true);
        let plan_id_value = data.plan_id.clone();
        let recipe_id_value = data.recipe.recipe_id.clone();
        spawn_local(async move {
            match preview_recipe_edit(&plan_id_value, &recipe_id_value, prompt).await {
                Ok(preview) => set_edit_preview.set(Some(preview)),
                Err(error) => set_edit_feedback.set(error),
            }
            set_preview_loading.set(false);
        });
    });

    let on_apply_edit = Callback::new(move |_| {
        let Some(preview) = edit_preview.get() else {
            return;
        };

        let plan_id_value = preview.plan_id.clone();
        let recipe_id_value = preview.recipe_id.clone();
        let recipe_value = preview.suggested_recipe.clone();

        set_edit_feedback.set(String::new());
        set_apply_loading.set(true);
        spawn_local(async move {
            match apply_recipe_edit(&plan_id_value, &recipe_id_value, recipe_value).await {
                Ok(_) => {
                    set_show_edit_modal.set(false);
                    set_edit_prompt.set(String::new());
                    set_edit_preview.set(None);
                    set_refresh_nonce.update(|value| *value += 1);
                }
                Err(error) => set_edit_feedback.set(error),
            }
            set_apply_loading.set(false);
        });
    });

    view! {
        <div class="min-h-screen bg-white dark:bg-background-dark text-black dark:text-white pb-32">
            <Suspense fallback=move || view! { <Loading /> }>
                {move || match recipe_resource.get() {
                    Some(Ok(data)) => {
                        let back_href = format!("/plan/{}", data.plan_id);
                        let ingredients = data.recipe.ingredients.clone();
                        let instructions = data.recipe.instructions.clone();
                        let notes = data.recipe.notes.clone();
                        let plan_instructions = data.plan_instructions.clone();
                        let meal_label = data.recipe.meal_type.display_name().to_string();
                        view! {
                            <div class="px-6 py-6 space-y-10">
                                <header class="border-b border-black dark:border-neutral-800 pb-8">
                                    <a href=back_href class="inline-flex items-center gap-2 text-[10px] font-black uppercase tracking-[0.2em] text-zinc-400 hover:text-black dark:hover:text-white transition-colors">
                                        <span class="material-icons-outlined text-base">arrow_back</span>
                                        "Volver al plan"
                                    </a>
                                    <div class="mt-8 flex items-center gap-3">
                                        <div class="w-10 h-[2px] bg-accent"></div>
                                        <span class="text-[10px] font-black uppercase tracking-[0.3em] text-zinc-400">
                                            {data.plan_title.clone()}
                                        </span>
                                    </div>
                                    <h1 class="mt-6 text-5xl md:text-7xl font-black uppercase tracking-tighter leading-[0.9]">
                                        {data.recipe.name.clone()}
                                    </h1>
                                    <div class="mt-6 flex flex-wrap gap-3">
                                        <div class="px-3 py-2 bg-black dark:bg-white text-white dark:text-black text-[10px] font-black uppercase tracking-[0.2em]">
                                            {data.day.label.clone()}
                                        </div>
                                        <div class="px-3 py-2 border border-black dark:border-neutral-700 text-[10px] font-black uppercase tracking-[0.2em]">
                                            {meal_label}
                                        </div>
                                    </div>
                                    <div class="mt-6 flex flex-wrap gap-3">
                                        <button
                                            on:click=open_edit_modal
                                            class="px-4 py-3 border border-black dark:border-white text-[10px] font-black uppercase tracking-[0.25em] hover:bg-black hover:text-white dark:hover:bg-white dark:hover:text-black transition-colors"
                                        >
                                            "Editar Con IA"
                                        </button>
                                        <button
                                            on:click=move |_| set_show_assign_modal.set(true)
                                            class="px-4 py-3 bg-accent text-neutral-950 text-[10px] font-black uppercase tracking-[0.25em] hover:brightness-95 transition-all"
                                        >
                                            "Asignar Semana Completa"
                                        </button>
                                    </div>
                                </header>

                                {if let Some(base_notes) = plan_instructions.filter(|text| !text.trim().is_empty()) {
                                    view! {
                                        <section class="space-y-3">
                                            <h2 class="text-xs font-black uppercase tracking-[0.2em] text-zinc-400">
                                                "Preparación Base Del Plan"
                                            </h2>
                                            <div class="border border-zinc-200 dark:border-neutral-800 p-5 text-sm leading-relaxed text-zinc-700 dark:text-zinc-200 whitespace-pre-line">
                                                {base_notes}
                                            </div>
                                        </section>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}

                                <section class="grid grid-cols-1 lg:grid-cols-[0.9fr_1.1fr] gap-6">
                                    <div class="border border-zinc-200 dark:border-neutral-800 p-5 space-y-4">
                                        <h2 class="text-xs font-black uppercase tracking-[0.2em] text-zinc-400">
                                            "Ingredientes"
                                        </h2>
                                        <ul class="space-y-3">
                                            {ingredients.into_iter().map(|ingredient| view! {
                                                {
                                                    let ingredient_value = ingredient.clone();
                                                    let ingredient_value_for_class = ingredient.clone();
                                                    let ingredient_label = ingredient.clone();
                                                    let ingredient_for_toggle = ingredient.clone();
                                                    view! {
                                                        <li>
                                                            <button
                                                                on:click=move |_| toggle_ingredient.run(ingredient_for_toggle.clone())
                                                                class="w-full flex items-start gap-3 text-sm leading-relaxed text-left"
                                                            >
                                                                <span class="material-icons-outlined text-accent text-base mt-0.5">
                                                                    {move || if checked_ingredients.get().contains(&ingredient_value) {
                                                                        "check_box"
                                                                    } else {
                                                                        "check_box_outline_blank"
                                                                    }}
                                                                </span>
                                                                <span class=move || {
                                                                    if checked_ingredients.get().contains(&ingredient_value_for_class) {
                                                                        "line-through text-zinc-400"
                                                                    } else {
                                                                        ""
                                                                    }
                                                                }>
                                                                    {ingredient_label.clone()}
                                                                </span>
                                                            </button>
                                                        </li>
                                                    }
                                                }
                                            }).collect_view()}
                                        </ul>
                                    </div>

                                    <div class="border border-zinc-200 dark:border-neutral-800 p-5 space-y-4">
                                        <h2 class="text-xs font-black uppercase tracking-[0.2em] text-zinc-400">
                                            "Modo De Preparación"
                                        </h2>
                                        {if instructions.is_empty() {
                                            view! {
                                                <div class="text-sm leading-relaxed text-zinc-500">
                                                    "Esta receta todavía no tiene pasos detallados guardados. Puedes editarla con IA desde el calendario para generar una preparación más específica."
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <ol class="space-y-4">
                                                    {instructions.into_iter().enumerate().map(|(index, step)| view! {
                                                        <li class="flex items-start gap-4">
                                                            <div class="w-8 h-8 flex items-center justify-center border border-black dark:border-white text-[10px] font-black uppercase flex-shrink-0">
                                                                {index + 1}
                                                            </div>
                                                            <p class="text-sm leading-relaxed text-zinc-700 dark:text-zinc-200">
                                                                {step}
                                                            </p>
                                                        </li>
                                                    }).collect_view()}
                                                </ol>
                                            }.into_any()
                                        }}
                                    </div>
                                </section>

                                {if let Some(extra_notes) = notes.filter(|text| !text.trim().is_empty()) {
                                    view! {
                                        <section class="border border-zinc-200 dark:border-neutral-800 p-5 space-y-3">
                                            <h2 class="text-xs font-black uppercase tracking-[0.2em] text-zinc-400">
                                                "Notas"
                                            </h2>
                                            <p class="text-sm leading-relaxed text-zinc-700 dark:text-zinc-200 whitespace-pre-line">
                                                {extra_notes}
                                            </p>
                                        </section>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                        }.into_any()
                    }
                    Some(Err(error)) => {
                        log!("Error loading recipe detail: {}", error);
                        let back_href = format!("/plan/{}", plan_id());
                        view! {
                            <div class="px-6 py-16 space-y-6">
                                <a href=back_href class="inline-flex items-center gap-2 text-[10px] font-black uppercase tracking-[0.2em] text-zinc-400 hover:text-black dark:hover:text-white transition-colors">
                                    <span class="material-icons-outlined text-base">arrow_back</span>
                                    "Volver al plan"
                                </a>
                                <div class="border border-dashed border-zinc-300 dark:border-neutral-700 p-6 text-sm text-zinc-500">
                                    {error}
                                </div>
                            </div>
                        }.into_any()
                    }
                    None => view! { <Loading /> }.into_any(),
                }}
            </Suspense>

            <footer class="fixed bottom-0 left-0 right-0 p-6 bg-white/80 dark:bg-background-dark/80 backdrop-blur-lg border-t border-neutral-100 dark:border-neutral-800 z-[45]">
                <button
                    on:click=move |_| set_show_assign_modal.set(true)
                    class="w-full bg-accent py-5 flex items-center justify-center gap-3 active:scale-[0.98] transition-transform text-neutral-950"
                >
                    <span class="text-sm font-bold uppercase tracking-[0.3em]">"Asignar Plan a La Semana"</span>
                    <span class="material-symbols-outlined !text-base">"calendar_month"</span>
                </button>
            </footer>

            {move || if show_assign_modal.get() {
                let next_monday = get_next_weekday(Weekday::Mon);
                let next_tuesday = get_next_weekday(Weekday::Tue);
                let next_monday_value = StoredValue::new(next_monday.clone());
                let next_tuesday_value = StoredValue::new(next_tuesday.clone());
                let next_monday_label =
                    StoredValue::new(format!("Próximo Lunes ({})", next_monday.clone()));
                let next_tuesday_label =
                    StoredValue::new(format!("Próximo Martes ({})", next_tuesday.clone()));
                view! {
                    <Portal>
                        <Modal on_close=Callback::new(move |_| set_show_assign_modal.set(false))>
                            <div class="space-y-6">
                                <div>
                                    <h2 class="text-2xl font-black uppercase tracking-tight text-neutral-950 dark:text-white">
                                        "Asignar Semana Completa"
                                    </h2>
                                    <p class="mt-2 text-[10px] font-bold uppercase tracking-[0.2em] text-neutral-400 dark:text-neutral-500">
                                        "Se asignarán automáticamente las recetas del plan a cada día correspondiente."
                                    </p>
                                </div>

                                <div class="flex flex-col gap-3">
                                    <button
                                        on:click=move |_| on_assign_weekly.run(next_monday_value.get_value())
                                        disabled=move || assigning_weekly.get()
                                        class="w-full p-4 border border-neutral-200 dark:border-neutral-700 text-left uppercase font-bold text-sm hover:border-neutral-950 dark:hover:border-white transition-colors disabled:opacity-50"
                                    >
                                        {next_monday_label.get_value()}
                                    </button>
                                    <button
                                        on:click=move |_| on_assign_weekly.run(next_tuesday_value.get_value())
                                        disabled=move || assigning_weekly.get()
                                        class="w-full p-4 border border-neutral-200 dark:border-neutral-700 text-left uppercase font-bold text-sm hover:border-neutral-950 dark:hover:border-white transition-colors disabled:opacity-50"
                                    >
                                        {next_tuesday_label.get_value()}
                                    </button>
                                    <a
                                        href="/calendar"
                                        class="w-full p-4 border border-neutral-200 dark:border-neutral-700 text-left uppercase font-bold text-sm hover:border-neutral-950 dark:hover:border-white transition-colors block"
                                    >
                                        "Ir al Calendario"
                                    </a>
                                </div>
                            </div>
                        </Modal>
                    </Portal>
                }.into_any()
            } else {
                ().into_any()
            }}

            {move || if show_edit_modal.get() {
                let preview = StoredValue::new(edit_preview.get());
                let current_recipe = StoredValue::new(
                    recipe_resource
                    .get()
                    .and_then(Result::ok)
                    .map(|data| data.recipe),
                );
                view! {
                    <Portal>
                        <Modal on_close=Callback::new(move |_| {
                            set_show_edit_modal.set(false);
                            set_edit_prompt.set(String::new());
                            set_edit_preview.set(None);
                            set_edit_feedback.set(String::new());
                        })>
                            <div class="space-y-6">
                                <div>
                                    <h2 class="text-2xl font-black uppercase tracking-tight text-neutral-950 dark:text-white">
                                        "Editar Receta Con IA"
                                    </h2>
                                    <p class="mt-2 text-[10px] font-bold uppercase tracking-[0.2em] text-neutral-400 dark:text-neutral-500">
                                        "El cambio se aplica solo a esta receta dentro del plan."
                                    </p>
                                </div>

                                {move || if !edit_feedback.get().is_empty() {
                                    view! {
                                        <div class="border border-red-200 bg-red-50 text-red-600 px-4 py-3 text-sm">
                                            {edit_feedback.get()}
                                        </div>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}

                                {if let Some(recipe) = current_recipe.get_value() {
                                    view! {
                                        <div class="border border-neutral-200 dark:border-neutral-700 p-4 space-y-2">
                                            <div class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-400">
                                                "Receta Actual"
                                            </div>
                                            <div class="text-lg font-black uppercase">
                                                {recipe.name}
                                            </div>
                                            <div class="text-sm text-neutral-600 dark:text-neutral-300">
                                                {recipe.ingredients.join(", ")}
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}

                                <textarea
                                    class="w-full min-h-28 border border-neutral-200 dark:border-neutral-700 bg-transparent px-4 py-3 text-sm outline-none"
                                    placeholder="Ejemplo: hazla más fácil de preparar, cambia una guarnición o detalla mejor los pasos."
                                    on:input=move |ev| set_edit_prompt.set(event_target_value(&ev))
                                    prop:value=edit_prompt
                                ></textarea>

                                <button
                                    on:click=move |_| on_preview_edit.run(())
                                    disabled=move || preview_loading.get()
                                    class="w-full py-3 bg-black dark:bg-white text-white dark:text-black text-[10px] font-black uppercase tracking-[0.3em] disabled:opacity-60"
                                >
                                    {move || if preview_loading.get() { "Generando Sugerencia" } else { "Vista Previa Con IA" }}
                                </button>

                                {if let Some(preview_value) = preview.get_value() {
                                    let suggested_recipe = preview_value.suggested_recipe.clone();
                                    view! {
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                            <div class="border border-neutral-200 dark:border-neutral-700 p-4 space-y-2">
                                                <div class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-400">
                                                    "Antes"
                                                </div>
                                                <div class="text-lg font-black uppercase">
                                                    {preview_value.original_recipe.name.clone()}
                                                </div>
                                                <div class="text-sm text-neutral-600 dark:text-neutral-300">
                                                    {preview_value.original_recipe.ingredients.join(", ")}
                                                </div>
                                            </div>
                                            <div class="border border-neutral-200 dark:border-neutral-700 p-4 space-y-2">
                                                <div class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-400">
                                                    "Después"
                                                </div>
                                                <div class="text-lg font-black uppercase">
                                                    {suggested_recipe.name}
                                                </div>
                                                <div class="text-sm text-neutral-600 dark:text-neutral-300">
                                                    {suggested_recipe.ingredients.join(", ")}
                                                </div>
                                            </div>
                                        </div>
                                        <button
                                            on:click=move |_| on_apply_edit.run(())
                                            disabled=move || apply_loading.get()
                                            class="w-full py-3 border border-black dark:border-white text-[10px] font-black uppercase tracking-[0.3em] disabled:opacity-60"
                                        >
                                            {move || if apply_loading.get() { "Aplicando Cambios" } else { "Confirmar Actualización" }}
                                        </button>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}
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
