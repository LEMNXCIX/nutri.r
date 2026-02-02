use crate::components::ui::{Button, Card, Loading, Modal, StarRating, TagBadge, TagSelector};
use crate::tauri_bridge::{
    add_tag_to_plan, assign_plan_to_date, calculate_nutrition, generate_variation, get_all_tags,
    get_calendar_range, get_plan_content, get_plan_metadata, remove_tag_from_plan, send_plan_email,
    set_plan_rating, toggle_favorite, MealType, PlanMetadata, Tag, VariationType,
};
use chrono::{Datelike, Local, Month, NaiveDate};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PlanDay {
    dia: String,
    comidas: Vec<PlanMeal>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PlanMeal {
    tipo: String,
    nombre: String,
    ingredientes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct StructuredPlan {
    titulo: String,
    instrucciones: Option<String>,
    dias: Vec<PlanDay>,
}

#[component]
pub fn PlanDetail() -> impl IntoView {
    let params = use_params_map();
    let id_signal = move || params.with(|params| params.get("id").unwrap_or_default());

    let (metadata, set_metadata) = signal(PlanMetadata::default());
    let (generating_variation, set_generating_variation) = signal(false);
    let navigate = leptos_router::hooks::use_navigate();
    let (email, set_email) = signal(String::new());
    let (show_email_input, set_show_email_input) = signal(false);
    let (sending_email, set_sending_email) = signal(false);

    // Calendar Assign State
    let (show_assign_modal, set_show_assign_modal) = signal(false);
    let now = Local::now().date_naive();
    let (assign_year, set_assign_year) = signal(now.year());
    let (assign_month, set_assign_month) = signal(now.month());
    let (selected_date_meal, set_selected_date_meal) =
        signal(Option::<(NaiveDate, MealType)>::None);

    let calendar_resource = LocalResource::new(move || {
        let year = assign_year.get();
        let month = assign_month.get();
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

    let nutrition_resource = LocalResource::new(move || {
        let id_val = id_signal();
        async move { calculate_nutrition(&id_val).await }
    });

    let (all_tags, set_all_tags) = signal::<Vec<Tag>>(vec![]);
    spawn_local(async move {
        if let Ok(tags) = get_all_tags().await {
            set_all_tags.set(tags);
        }
    });

    let (structured_plan, set_structured_plan) = signal::<Option<StructuredPlan>>(None);

    let on_add_tag = Callback::new(move |tag: Tag| {
        let id_val = id_signal();
        let tag_id = tag.id.clone();
        spawn_local(async move {
            if let Ok(_) = add_tag_to_plan(id_val, tag_id.clone()).await {
                set_metadata.update(|m| {
                    if !m.tags.contains(&tag_id) {
                        m.tags.push(tag_id);
                    }
                });
            }
        });
    });

    let on_remove_tag = Callback::new(move |tag_id: String| {
        let id_val = id_signal();
        let tid = tag_id.clone();
        spawn_local(async move {
            if let Ok(_) = remove_tag_from_plan(id_val, tid.clone()).await {
                set_metadata.update(|m| m.tags.retain(|t| t != &tid));
            }
        });
    });

    let plan_resource = LocalResource::new(move || {
        let id_val = id_signal();
        async move {
            let content_res = get_plan_content(&id_val).await;
            let meta_res = get_plan_metadata(&id_val).await;

            if let Ok(m) = meta_res {
                set_metadata.set(m);
            }

            match content_res {
                Ok(content) => {
                    // Try to parse as JSON first
                    let trimmed = content.trim();
                    if (trimmed.starts_with('{') && trimmed.ends_with('}'))
                        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
                    {
                        // Find the start and end of JSON if there is markdown fluff
                        let start = trimmed.find('{').or_else(|| trimmed.find('[')).unwrap_or(0);
                        let end = trimmed
                            .rfind('}')
                            .or_else(|| trimmed.rfind(']'))
                            .unwrap_or(trimmed.len() - 1);
                        let json_part = &trimmed[start..=end];

                        if let Ok(sp) = serde_json::from_str::<StructuredPlan>(json_part) {
                            set_structured_plan.set(Some(sp));
                            return Some("STRUCTURED".to_string());
                        } else if let Ok(dias) = serde_json::from_str::<Vec<PlanDay>>(json_part) {
                            set_structured_plan.set(Some(StructuredPlan {
                                titulo: "Plan Nutricional".to_string(),
                                instrucciones: None,
                                dias,
                            }));
                            return Some("STRUCTURED".to_string());
                        }
                    }

                    set_structured_plan.set(None);
                    let html = markdown::to_html(&content);
                    Some(html)
                }
                Err(err) => {
                    log!("Error fetching plan content: {}", err);
                    None
                }
            }
        }
    });

    let on_toggle_fav = move |_| {
        let id_val = id_signal();
        spawn_local(async move {
            if let Ok(fav) = toggle_favorite(&id_val).await {
                set_metadata.update(|m| m.is_favorite = fav);
            }
        });
    };

    let on_rate = Callback::new(move |rating: u8| {
        let id_val = id_signal();
        spawn_local(async move {
            if let Ok(_) = set_plan_rating(&id_val, rating).await {
                set_metadata.update(|m| m.rating = Some(rating));
            }
        });
    });

    let on_variation = Callback::new(move |v_type: VariationType| {
        let id_val = id_signal();
        let navigate = navigate.clone();
        set_generating_variation.set(true);
        spawn_local(async move {
            match generate_variation(&id_val, v_type).await {
                Ok(new_id) => {
                    navigate(&format!("/plan/{}", new_id), Default::default());
                    set_generating_variation.set(false);
                }
                Err(e) => {
                    log!("Error generating variation: {}", e);
                    set_generating_variation.set(false);
                }
            }
        });
    });

    let on_send_email = move |_| {
        let id_val = id_signal();
        let target = email.get();
        if target.is_empty() {
            return;
        }

        set_sending_email.set(true);
        spawn_local(async move {
            match send_plan_email(id_val, target).await {
                Ok(_) => {
                    set_show_email_input.set(false);
                }
                Err(e) => {
                    log!("Error sending email: {}", e);
                }
            }
            set_sending_email.set(false);
        });
    };

    view! {
        <div class="animate-in fade-in duration-500 bg-white min-h-screen font-sans text-gray-900">
            // Full Width Background Header
            <div class="bg-white border-b border-gray-200 pb-16 pt-8 px-4">
                <div class="max-w-5xl mx-auto">
                    <div class="flex flex-col md:flex-row md:items-end justify-between gap-6">
                        <div class="space-y-4">
                             <A href="/" attr:class="inline-flex items-center text-sm font-bold text-gray-400 hover:text-black transition-colors group">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1 group-hover:-translate-x-1 transition-transform" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z" clip-rule="evenodd" />
                                </svg>
                                "EXPLORAR PLANES"
                             </A>
                             <div class="space-y-1">
                                <span class="text-xs font-black text-gray-400 tracking-widest uppercase">"VISTA DE DETALLE"</span>
                                <h2 class="text-4xl md:text-5xl font-black text-black tracking-tighter">
                                    {move || if let Some(p) = structured_plan.get() { p.titulo } else { format!("Plan #{}", id_signal()) }}
                                </h2>
                             </div>
                        </div>

                        // Action Bar - Modern Floating Style
                        <div class="bg-white p-2 rounded-2xl border border-gray-200 shadow-sm flex flex-wrap items-center gap-2">
                            <div class="px-3 py-1 bg-gray-50 rounded-xl border border-gray-100">
                                <StarRating
                                    rating=Signal::derive(move || metadata.get().rating)
                                    on_rate=on_rate
                                />
                            </div>

                            <button
                                on:click=on_toggle_fav
                                class="p-2.5 rounded-xl bg-white border border-gray-200 hover:bg-black hover:text-white hover:border-black transition-all group"
                                title="Añadir a favoritos"
                            >
                                <svg
                                    class=move || if metadata.get().is_favorite {
                                        "w-6 h-6 text-red-600 fill-current"
                                    } else {
                                        "w-6 h-6 text-gray-400 fill-none group-hover:text-white transition-colors"
                                    }
                                    xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                                >
                                    <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
                                </svg>
                            </button>

                            <div class="h-8 w-px bg-gray-200 mx-1"></div>

                            <A
                                href=move || format!("/shopping/{}", id_signal())
                                attr:class="flex items-center gap-2 px-4 py-2.5 rounded-xl bg-white hover:bg-black hover:text-white text-gray-900 border border-gray-200 font-bold transition-all text-sm group"
                            >
                                <svg class="w-5 h-5 group-hover:scale-110 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                                </svg>
                                "LISTA COMPRAS"
                            </A>

                            <button
                                on:click=move |_| set_show_assign_modal.set(true)
                                class="flex items-center gap-2 px-4 py-2.5 rounded-xl bg-white hover:bg-black hover:text-white text-gray-900 border border-gray-200 font-bold transition-all text-sm group"
                            >
                                <svg class="w-5 h-5 group-hover:scale-110 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                                </svg>
                                "ASIGNAR A CALENDARIO"
                            </button>

                            <div class="relative group/ia">
                                <button class="flex items-center gap-2 px-4 py-2.5 rounded-xl bg-black text-white hover:bg-gray-800 border border-black font-bold transition-all text-sm shadow-lg shadow-black/20">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                                    "VARIANTES IA"
                                </button>
                                <div class="absolute right-0 mt-2 w-56 bg-white border border-gray-200 rounded-2xl shadow-xl opacity-0 group-hover/ia:opacity-100 invisible group-hover/ia:visible transition-all z-50 overflow-hidden">
                                    <div class="p-3 text-[10px] text-gray-400 uppercase font-black tracking-widest bg-gray-50 border-b border-gray-100">"Adaptar Plan"</div>
                                    <button on:click={let on_variation = on_variation.clone(); move |_| on_variation.run(VariationType::Vegan)} class="w-full text-left px-4 py-2.5 text-sm text-gray-600 hover:bg-gray-50 hover:text-black transition-colors flex items-center gap-3"><span>"🌱"</span> "Vegano"</button>
                                    <button on:click={let on_variation = on_variation.clone(); move |_| on_variation.run(VariationType::Keto)} class="w-full text-left px-4 py-2.5 text-sm text-gray-600 hover:bg-gray-50 hover:text-black transition-colors flex items-center gap-3"><span>"🥩"</span> "Keto"</button>
                                    <button on:click={let on_variation = on_variation.clone(); move |_| on_variation.run(VariationType::GlutenFree)} class="w-full text-left px-4 py-2.5 text-sm text-gray-600 hover:bg-gray-50 hover:text-black transition-colors flex items-center gap-3"><span>"🌾"</span> "Sin Gluten"</button>
                                    <button on:click={let on_variation = on_variation.clone(); move |_| on_variation.run(VariationType::HighProtein)} class="w-full text-left px-4 py-2.5 text-sm text-gray-600 hover:bg-gray-50 hover:text-black transition-colors flex items-center gap-3"><span>"💪"</span> "Alto en Proteína"</button>
                                </div>
                            </div>

                            <div class="relative group/mail">
                                <button
                                    on:click=move |_| set_show_email_input.update(|v| *v = !*v)
                                    class="p-2.5 rounded-xl bg-white border border-gray-200 text-gray-400 hover:text-black hover:border-black transition-all group"
                                >
                                    <svg class="w-6 h-6 group-hover:rotate-12 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
                                </button>

                                {move || if show_email_input.get() {
                                    view! {
                                        <div class="absolute right-0 mt-3 w-72 bg-white border border-gray-200 rounded-2xl shadow-xl p-4 z-50 space-y-4 animate-in slide-in-from-top-2">
                                            <div class="flex items-center gap-2">
                                                 <span class="text-xs font-black text-gray-900 uppercase tracking-wider">"Enviar por Correo"</span>
                                            </div>
                                            <input
                                                type="email"
                                                placeholder="correo@ejemplo.com"
                                                class="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-900 focus:ring-2 focus:ring-black focus:border-black outline-none transition-all"
                                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                                prop:value=email
                                            />
                                            <button
                                                on:click=on_send_email
                                                disabled=sending_email
                                                class="w-full bg-black hover:bg-gray-800 disabled:bg-gray-200 disabled:text-gray-400 text-white text-sm font-black py-3 rounded-xl transition-all shadow-lg active:scale-95"
                                            >
                                                {move || if sending_email.get() { "PROCESANDO..." } else { "ENVIAR AHORA" }}
                                            </button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div/> }.into_any()
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Main Content Area
            <div class="max-w-5xl mx-auto px-4 -mt-10 pb-20">
                <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
                    // Left Column: Plan Content
                    <div class="lg:col-span-8 space-y-8">
                        // Tags
                        <div class="flex flex-wrap items-center gap-2">
                            {move || metadata.get().tags.into_iter().map(|tag_id| {
                                let tid = tag_id.clone();
                                let tag_info = all_tags.get().into_iter().find(|t| t.id == tag_id);
                                match tag_info {
                                    Some(t) => view! {
                                        <TagBadge
                                            name=t.name
                                            color=t.color
                                            on_remove=Callback::new(move |_| on_remove_tag.run(tid.clone()))
                                        />
                                    }.into_any(),
                                    None => view! {
                                        <TagBadge
                                            name=tag_id.clone()
                                            color="#94a3b8".to_string()
                                            on_remove=Callback::new(move |_| on_remove_tag.run(tag_id.clone()))
                                        />
                                    }.into_any()
                                }
                            }).collect::<Vec<_>>()}
                            <TagSelector
                                on_select=on_add_tag
                                existing_tag_ids=Signal::derive(move || metadata.get().tags)
                            />
                        </div>

                        // Content Renderer
                        <Suspense fallback=move || view! { <Loading /> }>
                            {move || {
                                let content_html = plan_resource.get().flatten();
                                match content_html {
                                    Some(html) if html == "STRUCTURED" => {
                                        let sp = structured_plan.get().unwrap();
                                        view! {
                                            <div class="space-y-8">
                                                {move || sp.instrucciones.as_ref().map(|inst| {
                                                    let inst = inst.clone();
                                                    view! {
                                                        <div class="bg-blue-50 p-6 rounded-3xl border border-blue-100 italic text-blue-900 text-sm leading-relaxed">
                                                            {format!("💡 {}", inst)}
                                                        </div>
                                                    }
                                                })}

                                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                                    {sp.dias.into_iter().map(|day| {
                                                        view! {
                                                            <Card class="p-6 bg-white border-gray-100 shadow-sm hover:shadow-md transition-shadow">
                                                                <h3 class="text-xl font-black text-gray-900 mb-6 uppercase tracking-tight border-b border-gray-100 pb-2 flex items-center justify-between">
                                                                     {day.dia}
                                                                     <span class="text-[10px] text-gray-400 font-normal normal-case tracking-normal">"Día del plan"</span>
                                                                </h3>
                                                                <div class="space-y-6">
                                                                    {day.comidas.into_iter().map(|meal| {
                                                                        view! {
                                                                            <div class="group/meal transition-all">
                                                                                <div class="flex items-center gap-2 mb-2">
                                                                                    <span class="text-[10px] font-black text-gray-900 uppercase px-2 py-0.5 bg-gray-100 rounded-md border border-gray-200">
                                                                                        {meal.tipo}
                                                                                    </span>
                                                                                    <span class="text-sm font-bold text-gray-800 group-hover/meal:text-black transition-colors">
                                                                                        {meal.nombre}
                                                                                    </span>
                                                                                </div>
                                                                                <ul class="ml-4 space-y-1">
                                                                                    {meal.ingredientes.into_iter().map(|ing| view! {
                                                                                        <li class="text-xs text-gray-500 flex items-center gap-2">
                                                                                            <span class="w-1 h-1 bg-gray-300 rounded-full"></span>
                                                                                            {ing}
                                                                                        </li>
                                                                                    }).collect::<Vec<_>>()}
                                                                                </ul>
                                                                            </div>
                                                                        }
                                                                    }).collect::<Vec<_>>()}
                                                                </div>
                                                            </Card>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                    Some(html) => view! {
                                        <article
                                            class="prose prose-gray max-w-none bg-white p-8 md:p-12 rounded-3xl shadow-sm border border-gray-100"
                                            prop:innerHTML=html
                                        />
                                    }.into_any(),
                                    None => view! {
                                        <div class="text-center p-20 bg-gray-50 rounded-3xl border border-dashed border-gray-200">
                                            <Loading />
                                            <p class="text-gray-500 mt-6 font-medium">"Diseñando tu plan a medida..."</p>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </Suspense>
                    </div>

                    // Right Column: Side Info
                    <div class="lg:col-span-4 space-y-8">
                        // Nutrition Analysis
                        <Suspense fallback=move || view! { <Loading /> }>
                            {move || nutrition_resource.get().map(|res| match res {
                                Ok(n) if n.total_calories > 1.0 => {
                                    view! {
                                        <Card class="p-8 bg-white border-gray-100 shadow-sm relative overflow-hidden group">
                                            <div class="absolute top-0 right-0 w-32 h-32 bg-gray-50 -mr-16 -mt-16 rounded-full group-hover:scale-110 transition-transform duration-500"></div>

                                            <h3 class="text-lg font-black text-gray-900 mb-8 flex items-center gap-3 relative z-10">
                                                <div class="p-2 bg-gray-100 rounded-lg text-gray-900">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                                                </div>
                                                "NUTRICIÓN"
                                            </h3>

                                            <div class="space-y-6 relative z-10">
                                                <div class="flex items-end justify-between border-b border-gray-100 pb-4">
                                                     <div class="text-3xl font-black text-black leading-none">{n.total_calories as i32}</div>
                                                     <div class="text-xs font-black text-gray-400 uppercase tracking-widest">"Calorías / Día"</div>
                                                </div>

                                                <div class="grid grid-cols-3 gap-2">
                                                    <div class="p-3 bg-gray-50 rounded-2xl border border-gray-100 text-center">
                                                        <div class="text-[10px] font-black text-gray-400 uppercase mb-1">"Prot"</div>
                                                        <div class="text-xl font-black text-gray-900">{format!("{}g", n.total_protein as i32)}</div>
                                                    </div>
                                                    <div class="p-3 bg-gray-50 rounded-2xl border border-gray-100 text-center">
                                                        <div class="text-[10px] font-black text-gray-400 uppercase mb-1">"Carb"</div>
                                                        <div class="text-xl font-black text-gray-900">{format!("{}g", n.total_carbs as i32)}</div>
                                                    </div>
                                                    <div class="p-3 bg-gray-50 rounded-2xl border border-gray-100 text-center">
                                                        <div class="text-[10px] font-black text-gray-400 uppercase mb-1">"Gras"</div>
                                                        <div class="text-xl font-black text-gray-900">{format!("{}g", n.total_fat as i32)}</div>
                                                    </div>
                                                </div>

                                                {if !n.breakdown_by_item.is_empty() {
                                                    let mut items: Vec<_> = n.breakdown_by_item.into_iter().collect();
                                                    items.sort_by(|a, b| b.1.calories.partial_cmp(&a.1.calories).unwrap_or(std::cmp::Ordering::Equal));

                                                    view! {
                                                        <div class="pt-6 space-y-3">
                                                            <div class="text-xs font-black text-gray-400 uppercase tracking-widest px-1">"Principales Fuentes"</div>
                                                            <div class="space-y-2 max-h-[300px] overflow-y-auto pr-2 custom-scrollbar">
                                                                {items.into_iter().map(|(item, info)| {
                                                                    let item = item.clone();
                                                                    view! {
                                                                        <div class="flex justify-between items-center p-3 bg-gray-50 rounded-xl text-sm border border-gray-100 hover:border-gray-200 transition-colors">
                                                                            <span class="text-gray-900 font-bold capitalize truncate max-w-[120px]">{item}</span>
                                                                            <span class="text-[10px] font-mono text-gray-500">{format!("{} kcal", info.calories as i32)}</span>
                                                                        </div>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                } else { view! { <div/> }.into_any() }}
                                            </div>
                                        </Card>
                                    }.into_any()
                                },
                                _ => view! { <div/> }.into_any()
                            })}
                        </Suspense>

                        // Notes
                        <Card class="p-8 bg-white border-gray-100 shadow-sm">
                            <h3 class="text-lg font-black text-gray-900 mb-6 flex items-center gap-3">
                                <div class="p-2 bg-gray-100 rounded-lg text-gray-900">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" /></svg>
                                </div>
                                "NOTAS"
                            </h3>
                            <textarea
                                class="w-full bg-gray-50 shadow-inner rounded-2xl p-4 text-gray-900 focus:ring-2 focus:ring-black/10 border border-gray-100 focus:border-black transition-all outline-none resize-none min-h-[160px] text-sm leading-relaxed placeholder:text-gray-400"
                                placeholder="Escribe tus impresiones o ajustes aquí..."
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    let id_val = id_signal();
                                    spawn_local(async move {
                                        let _ = crate::tauri_bridge::set_plan_note(&id_val, val).await;
                                    });
                                }
                                prop:value=move || metadata.get().notes
                            />
                            <p class="text-[10px] text-gray-400 mt-4 text-center tracking-widest font-black uppercase">"Guardado automático"</p>
                        </Card>
                    </div>
                </div>
            </div>

            // Variation Loader Overlay
            {move || if generating_variation.get() {
                view! {
                    <div class="fixed inset-0 bg-white/90 backdrop-blur-md z-[100] flex flex-col items-center justify-center text-center p-8">
                        <div class="bg-white p-12 rounded-[3rem] border border-gray-200 shadow-2xl space-y-8 max-w-lg">
                            <div class="relative">
                                <Loading size="h-16 w-16" />
                            </div>
                            <div class="space-y-2">
                                <h3 class="text-3xl font-black text-black tracking-tighter">"TRANSFORMANDO PLAN"</h3>
                                <p class="text-gray-500 font-medium">"Nuestra IA está adaptando cada ingrediente a tu nuevo estilo de vida..."</p>
                            </div>
                            <div class="bg-gray-50 p-4 rounded-2xl text-[10px] text-gray-500 uppercase font-black tracking-[0.2em]">
                                "PROCESANDO MODELO LOCAL"
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}

            // Assign to Calendar Modal
            {move || if show_assign_modal.get() {
                let on_close = move |_| {
                    set_show_assign_modal.set(false);
                    set_selected_date_meal.set(None);
                };

                let on_confirm_assignment = move |_| {
                    if let Some((date, meal)) = selected_date_meal.get() {
                        let id_val = id_signal();
                        let date_str = date.to_string();
                        spawn_local(async move {
                            if let Ok(_) = assign_plan_to_date(date_str, meal, id_val).await {
                                set_show_assign_modal.set(false);
                                set_selected_date_meal.set(None);
                                // Optional: Show toast or feedback
                            }
                        });
                    }
                };

                let on_prev_month = move |_| {
                    if assign_month.get() == 1 {
                        set_assign_month.set(12);
                        set_assign_year.update(|y| *y -= 1);
                    } else {
                        set_assign_month.update(|m| *m -= 1);
                    }
                };

                let on_next_month = move |_| {
                    if assign_month.get() == 12 {
                        set_assign_month.set(1);
                        set_assign_year.update(|y| *y += 1);
                    } else {
                        set_assign_month.update(|m| *m += 1);
                    }
                };

                view! {
                    <div class="fixed inset-0 bg-white/90 backdrop-blur-xl z-[200] flex items-center justify-center p-4 animate-in fade-in duration-300">
                        <Card class="max-w-2xl w-full p-8 bg-white rounded-[2.5rem] border border-gray-200 shadow-2xl relative overflow-hidden flex flex-col md:flex-row gap-8">

                            // Calendar Side
                            <div class="flex-1">
                                <header class="mb-6 flex items-center justify-between">
                                    <h3 class="text-xl font-black text-black tracking-tight uppercase">
                                        {move || format!("{} {}", Month::try_from(assign_month.get() as u8).ok().map(|m| m.name()).unwrap_or(""), assign_year.get())}
                                    </h3>
                                    <div class="flex items-center gap-1 bg-gray-50 rounded-lg p-1 border border-gray-100">
                                        <button on:click=on_prev_month class="p-1 hover:bg-white rounded-md text-gray-400 hover:text-black transition-all shadow-sm"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg></button>
                                        <button on:click=on_next_month class="p-1 hover:bg-white rounded-md text-gray-400 hover:text-black transition-all shadow-sm"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg></button>
                                    </div>
                                </header>

                                <div class="grid grid-cols-7 mb-2">
                                     {vec!["L", "M", "M", "J", "V", "S", "D"].into_iter().map(|day| view! {
                                        <div class="text-center text-[10px] font-bold text-gray-400 pb-2">{day}</div>
                                    }).collect::<Vec<_>>()}
                                </div>
                                <div class="grid grid-cols-7 gap-1">
                                    <Suspense fallback=move || view! { <div class="col-span-7 flex justify-center py-8"><Loading size="w-6 h-6"/></div> }>
                                        {move || {
                                            let entries = calendar_resource.get().unwrap_or_default();
                                            let mut entries_map = HashMap::new();
                                            for e in entries {
                                                entries_map.entry(e.date.clone()).or_insert_with(Vec::new).push(e);
                                            }

                                            let year = assign_year.get();
                                            let month = assign_month.get();
                                            let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                                            let weekday = first_day.weekday().number_from_monday();
                                            let days_in_month = if month == 12 {
                                                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().pred_opt().unwrap().day()
                                            } else {
                                                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().pred_opt().unwrap().day()
                                            };

                                            let mut grid_items = Vec::new();
                                            for _ in 1..weekday {
                                                grid_items.push(view! { <div class="aspect-square"></div> }.into_any());
                                            }

                                            for d in 1..=days_in_month {
                                                let date = NaiveDate::from_ymd_opt(year, month, d).unwrap();
                                                let date_str = date.to_string();
                                                let day_entries = entries_map.get(&date_str).cloned().unwrap_or_default();
                                                let is_selected = selected_date_meal.get().map(|(sd, _)| sd == date).unwrap_or(false);
                                                let is_today = date == now;

                                                let has_meals = !day_entries.is_empty();

                                                let on_select = move |_| {
                                                     set_selected_date_meal.update(|v| {
                                                        if let Some((_d, m)) = v {
                                                            *v = Some((date.clone(), m.clone()));
                                                        } else {
                                                            *v = Some((date.clone(), MealType::Lunch)); // Default
                                                        }
                                                     });
                                                };

                                                grid_items.push(view! {
                                                    <button
                                                        on:click=on_select
                                                        class=format!("aspect-square rounded-xl flex flex-col items-center justify-center relative transition-all border {}",
                                                            if is_selected { "bg-black text-white border-black shadow-lg shadow-black/20 scale-105" }
                                                            else if is_today { "bg-gray-50 text-black border-black/20" }
                                                            else { "bg-white text-gray-500 hover:bg-gray-50 border-transparent hover:border-gray-200" }
                                                        )
                                                    >
                                                        <span class="text-xs font-bold">{d}</span>
                                                        {if has_meals {
                                                            view! { <div class=format!("w-1 h-1 rounded-full mt-1 {}", if is_selected { "bg-white/50" } else { "bg-black/80" })></div> }.into_any()
                                                        } else {
                                                            ().into_any()
                                                        }}
                                                    </button>
                                                }.into_any());
                                            }
                                            grid_items
                                        }}
                                    </Suspense>
                                </div>
                            </div>

                            // Selection Side
                            <div class="md:w-64 flex flex-col border-t md:border-t-0 md:border-l border-gray-100 pt-6 md:pt-0 md:pl-8">
                                <h4 class="text-sm font-black text-gray-400 uppercase tracking-widest mb-4">"Selección"</h4>

                                {move || if let Some((date, meal)) = selected_date_meal.get() {
                                    view! {
                                        <div class="flex-1 flex flex-col gap-4 animate-in fade-in slide-in-from-right-4">
                                            <div>
                                                <div class="text-3xl font-black text-black tracking-tighter mb-1">
                                                     {date.day()} " " {Month::try_from(date.month() as u8).ok().map(|m| m.name()).unwrap_or("")}
                                                </div>
                                                <div class="text-xs font-mono text-gray-400">{date.year()}</div>
                                            </div>

                                            <div class="space-y-2">
                                                <label class="text-[10px] font-bold text-gray-400 uppercase">"Tipo de Comida"</label>
                                                <div class="grid grid-cols-1 gap-2">
                                                    {[MealType::Breakfast, MealType::Lunch, MealType::Dinner, MealType::Snack].into_iter().map(|m| {
                                                        let is_active = m == meal;
                                                        let m_clone = m.clone();
                                                        let label = match m {
                                                            MealType::Breakfast => "Desayuno",
                                                            MealType::Lunch => "Almuerzo",
                                                            MealType::Dinner => "Cena",
                                                            MealType::Snack => "Merienda",
                                                        };
                                                        let icon = match m {
                                                            MealType::Breakfast => "☀️",
                                                            MealType::Lunch => "🥘",
                                                            MealType::Dinner => "🌙",
                                                            MealType::Snack => "🍎",
                                                        };

                                                        view! {
                                                            <button
                                                                on:click=move |_| set_selected_date_meal.set(Some((date.clone(), m_clone.clone())))
                                                                class=format!("w-full text-left px-4 py-3 rounded-xl border flex items-center justify-between transition-all {}",
                                                                    if is_active { "bg-black text-white border-black shadow-lg" }
                                                                    else { "bg-white text-gray-500 border-gray-100 hover:border-gray-200 hover:bg-gray-50" }
                                                                )
                                                            >
                                                                <span class="font-bold text-sm">{label}</span>
                                                                <span>{icon}</span>
                                                            </button>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>

                                            <div class="mt-auto pt-4 flex flex-col gap-2">
                                                <Button on_click=Callback::new(on_confirm_assignment) class="w-full bg-black text-white hover:bg-gray-900 h-12 rounded-xl shadow-xl shadow-black/10">"Confirmar Asignación"</Button>
                                                <button on:click=on_close class="w-full h-10 text-xs font-bold text-gray-400 hover:text-black uppercase tracking-widest">"Cancelar"</button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="flex-1 flex flex-col items-center justify-center text-center text-gray-400 p-4 border-2 border-dashed border-gray-100 rounded-2xl">
                                            <svg class="w-8 h-8 mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
                                            <p class="text-sm font-medium">"Selecciona un día en el calendario"</p>
                                        </div>
                                        <div class="mt-auto pt-4">
                                            <button on:click=on_close class="w-full h-10 text-xs font-bold text-gray-400 hover:text-black uppercase tracking-widest">"Cancelar"</button>
                                        </div>
                                    }.into_any()
                                }}
                            </div>
                        </Card>
                    </div>
                }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}
        </div>
    }
}
