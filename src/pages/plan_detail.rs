use crate::components::ui::Loading;
use crate::tauri_bridge::{
    add_tag_to_plan, calculate_nutrition, generate_variation, get_all_tags, get_plan_content,
    get_plan_metadata, remove_tag_from_plan, send_plan_email, set_plan_rating, toggle_favorite,
    PlanMetadata, Tag, VariationType,
};
use leptos::logging::log;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};

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

                    // Full Markdown support (Tables, Task lists, Strikethrough, etc.)
                    let mut options = pulldown_cmark::Options::empty();
                    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
                    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
                    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
                    options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
                    options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);

                    let parser = pulldown_cmark::Parser::new_ext(&content, options);
                    let mut html = String::new();
                    pulldown_cmark::html::push_html(&mut html, parser);

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

    let on_send_email = move |_: web_sys::MouseEvent| {
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
        <div class="w-full font-sans pb-32">
            // -- HEADER SECTION --
            <header class="relative h-[60vh] w-full overflow-hidden">
                <img
                    alt="Nutritional Plan"
                    class="w-full h-full object-cover grayscale brightness-75"
                    src="https://lh3.googleusercontent.com/aida-public/AB6AXuCuLIs4J3BB-Asz5cdNOorESMj1X3AVHQ_CyacDzU2zpMKJ4AmCCVsAedD5NzL-tBYxXv2eygd4hFNASqgdKD0gQnv78equgwci1mxJTvwA2XoV8I5GKSnShEzhTNk-Sfq7lK0QTcqEUsgGCWjJnyFLnU1YJVwoIJEK5Hfo3fFegV_Qf78T58vwbdtEQOflSZsT_ZYtWI8zXgmyhEojqt3UqYpvZwNrIO1VYttV3E3A3lfStG6x_jIYbQxMszgc2jS4Z_ticQKZ8Mha"
                />

                <div class="absolute top-0 left-0 right-0 p-6 flex justify-between items-center z-10">
                    <A href="/" attr:class="w-10 h-10 bg-white flex items-center justify-center rounded-full shadow-sm">
                        <span class="material-symbols-outlined">"arrow_back"</span>
                    </A>
                    <button
                        on:click=on_toggle_fav
                        class="w-10 h-10 bg-white flex items-center justify-center rounded-full shadow-sm"
                    >
                        <span class=move || format!("material-symbols-outlined {}",
                            if metadata.get().is_favorite { "text-red-500" } else { "text-neutral-400" }
                        )>
                            "favorite"
                        </span>
                    </button>
                </div>

                <div class="absolute bottom-12 left-6 right-6">
                    <div class="bg-accent px-2 py-1 inline-block mb-4">
                        <span class="text-[10px] font-bold uppercase tracking-[0.2em]">"Plan de Autor"</span>
                    </div>
                    <h1 class="text-6xl font-extrabold text-white uppercase leading-[0.85] tracking-tighter">
                        {move || {
                            let title = if let Some(p) = structured_plan.get() {
                                p.titulo
                            } else {
                                format!("Plan Nutricional #{}", id_signal().chars().take(4).collect::<String>())
                            };
                            let words: Vec<&str> = title.split_whitespace().collect();
                            if words.len() >= 2 {
                                view! { {words[0]} <br/> {words[1..].join(" ")} }.into_any()
                            } else {
                                title.into_any()
                            }
                        }}
                    </h1>
                </div>
            </header>

            // -- NUTRITION SECTION --
            <section class="bg-white dark:bg-background-dark px-6 py-8">
                <Suspense fallback=move || view! { <div class="animate-pulse h-20 bg-neutral-100 dark:bg-neutral-900 mb-4"></div> }>
                    {move || {
                        let res = nutrition_resource.get();
                        let nutrition = res.and_then(|r| r.ok());

                        view! {
                            <div class="flex flex-col space-y-4">
                                <div class="flex justify-between items-center pb-4 border-b border-neutral-100 dark:border-neutral-800">
                                    <span class="text-[10px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Calorías Totales"</span>
                                    {match nutrition.as_ref() {
                                        Some(n) => view! { <span class="text-2xl font-light tracking-tighter">{format!("{} kcal", n.total_calories as i32)}</span> }.into_any(),
                                        None => view! { <span class="text-2xl font-light tracking-tighter text-neutral-200">"-- kcal"</span> }.into_any(),
                                    }}
                                </div>
                                <div class="grid grid-cols-3 gap-8 py-2">
                                    <div class="flex flex-col gap-1">
                                        <span class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Proteína"</span>
                                        <div class="flex items-baseline gap-1">
                                            {match nutrition.as_ref() {
                                                Some(n) => view! { <span class="text-2xl font-medium tracking-tighter">{n.total_protein as i32}</span> }.into_any(),
                                                None => view! { <span class="text-2xl font-medium tracking-tighter text-neutral-200">"--"</span> }.into_any(),
                                            }}
                                            <span class="text-[10px] font-bold text-neutral-400 uppercase">"g"</span>
                                        </div>
                                    </div>
                                    <div class="flex flex-col gap-1">
                                        <span class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Carbohidratos"</span>
                                        <div class="flex items-baseline gap-1">
                                            {match nutrition.as_ref() {
                                                Some(n) => view! { <span class="text-2xl font-medium tracking-tighter">{n.total_carbs as i32}</span> }.into_any(),
                                                None => view! { <span class="text-2xl font-medium tracking-tighter text-neutral-200">"--"</span> }.into_any(),
                                            }}
                                            <span class="text-[10px] font-bold text-neutral-400 uppercase">"g"</span>
                                        </div>
                                    </div>
                                    <div class="flex flex-col gap-1">
                                        <span class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Grasas"</span>
                                        <div class="flex items-baseline gap-1">
                                            {match nutrition.as_ref() {
                                                Some(n) => view! { <span class="text-2xl font-medium tracking-tighter">{n.total_fat as i32}</span> }.into_any(),
                                                None => view! { <span class="text-2xl font-medium tracking-tighter text-neutral-200">"--"</span> }.into_any(),
                                            }}
                                            <span class="text-[10px] font-bold text-neutral-400 uppercase">"g"</span>
                                        </div>
                                    </div>
                                </div>
                                <div class="hairline-divider dark:bg-neutral-800"></div>
                            </div>
                        }
                    }}
                </Suspense>
            </section>

            // -- ENHANCED ACTIONS SECTION --
            <section class="px-6 py-6 bg-neutral-50 dark:bg-neutral-900/30 space-y-8">
                // Quick Actions
                <div class="grid grid-cols-2 gap-4">
                    <A href=move || format!("/shopping/{}", id_signal())
                        attr:class="p-4 brutalist-border bg-white dark:bg-neutral-800 flex flex-col items-center justify-center gap-2 hover:bg-accent dark:hover:bg-accent hover:text-black transition-all group"
                    >
                        <span class="material-symbols-outlined text-2xl group-hover:scale-110 transition-transform">"shopping_cart"</span>
                        <span class="text-[10px] font-black uppercase tracking-widest">"Lista de Compras"</span>
                    </A>
                    <button
                        on:click=move |_| set_show_email_input.update(|v| *v = !*v)
                        class="p-4 brutalist-border bg-white dark:bg-neutral-800 flex flex-col items-center justify-center gap-2 hover:bg-accent dark:hover:bg-accent hover:text-black transition-all group"
                    >
                        <span class="material-symbols-outlined text-2xl group-hover:scale-110 transition-transform">"mail"</span>
                        <span class="text-[10px] font-black uppercase tracking-widest">"Enviar Email"</span>
                    </button>
                </div>

                // Email Input (Conditional)
                {move || if show_email_input.get() {
                    view! {
                        <div class="p-4 bg-white dark:bg-neutral-800 brutalist-border flex items-center gap-2">
                            <input
                                type="email"
                                placeholder="tu@email.com"
                                class="flex-1 bg-transparent border-none outline-none text-sm font-medium"
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                prop:value=email
                            />
                            <button
                                on:click=on_send_email
                                disabled=move || sending_email.get()
                                class="bg-black dark:bg-white text-white dark:text-black px-4 py-2 text-[10px] font-black uppercase tracking-widest disabled:opacity-50"
                            >
                                {move || if sending_email.get() { "Enviando..." } else { "Enviar" }}
                            </button>
                        </div>
                    }.into_any()
                } else { ().into_any() }}

                // Rating
                <div class="flex flex-col gap-3">
                    <span class="text-[10px] font-black uppercase tracking-widest text-neutral-400">"Califica este Plan"</span>
                    <div class="flex gap-2">
                        {(1..=5).map(|i| {
                            let rated = move || metadata.get().rating.unwrap_or(0) >= i;
                            view! {
                                <button
                                    on:click=move |_| on_rate.run(i)
                                    class=move || format!("p-2 transition-colors {}", if rated() { "text-accent" } else { "text-neutral-200 dark:text-neutral-800" })
                                >
                                    <span class="material-symbols-outlined !text-4xl" style=move || if rated() { "font-variation-settings: 'FILL' 1" } else { "" }>
                                        "star"
                                    </span>
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Tags
                <div class="flex flex-col gap-4">
                    <div class="flex justify-between items-center">
                        <span class="text-[10px] font-black uppercase tracking-widest text-neutral-400">"Etiquetas"</span>
                    </div>

                    // Existing Tags
                    <div class="flex flex-wrap gap-2">
                        {move || {
                            let m = metadata.get();
                            let current_tags = m.tags.clone();
                            let all = all_tags.get();

                            current_tags.into_iter().map(|tid| {
                                let tag_name = all.iter().find(|t| t.id == tid).map(|t| t.name.clone()).unwrap_or(tid.clone());
                                let tid_c = tid.clone();
                                view! {
                                    <div class="px-3 py-1 bg-neutral-100 dark:bg-neutral-800 brutalist-border flex items-center gap-2 group">
                                        <span class="text-[10px] font-black uppercase">{tag_name}</span>
                                        <button on:click=move |_| on_remove_tag.run(tid_c.clone()) class="material-symbols-outlined !text-[14px] hover:text-red-500">"close"</button>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>

                    // Add Tag
                    <div class="flex flex-wrap gap-2 pt-2">
                        {move || {
                            let m = metadata.get();
                            let all = all_tags.get();
                            all.into_iter()
                                .filter(|t| !m.tags.contains(&t.id))
                                .map(|t| {
                                    let t_c = t.clone();
                                    view! {
                                        <button
                                            on:click=move |_| on_add_tag.run(t_c.clone())
                                            class="px-2 py-1 border border-dashed border-neutral-300 dark:border-neutral-700 text-[9px] font-bold uppercase text-neutral-400 hover:border-accent hover:text-accent transition-colors"
                                        >
                                            {format!("+ {}", t.name)}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>

                // Variations
                <div class="flex flex-col gap-3">
                    <span class="text-[10px] font-black uppercase tracking-widest text-neutral-400">"Generar Variación"</span>
                    <div class="flex gap-2">
                        <button
                            on:click=move |_| on_variation.run(VariationType::HighProtein)
                            class="flex-1 py-3 text-[10px] font-black uppercase tracking-widest brutalist-border bg-white dark:bg-neutral-800 hover:bg-neutral-950 dark:hover:bg-white hover:text-white dark:hover:text-black transition-all"
                        >
                            "Más Saludable"
                        </button>
                        <button
                            on:click=move |_| on_variation.run(VariationType::LowCarb)
                            class="flex-1 py-3 text-[10px] font-black uppercase tracking-widest brutalist-border bg-white dark:bg-neutral-800 hover:bg-neutral-950 dark:hover:bg-white hover:text-white dark:hover:text-black transition-all"
                        >
                            "Más Económico"
                        </button>
                    </div>
                </div>
            </section>

            // -- CONTENT SECTION --
            <section class="px-6 py-4">
                <Suspense fallback=move || view! { <Loading /> }>
                    {move || {
                        let content_html = plan_resource.get().flatten();
                        match content_html {
                            Some(html) if html == "STRUCTURED" => {
                                let sp = structured_plan.get().unwrap();
                                view! {
                                    <div class="space-y-12">
                                        <h2 class="text-xs font-bold uppercase tracking-[0.2em] mb-8 text-neutral-400">"Desglose del Plan Seleccionado"</h2>

                                        {if let Some(instr) = sp.instrucciones {
                                            view! {
                                                <div class="prose-brutalist mb-12" inner_html=instr />
                                            }.into_any()
                                        } else {
                                            ().into_any()
                                        }}

                                        <div class="space-y-12">
                                            {sp.dias.into_iter().map(|day| view! {
                                                <div class="space-y-6">
                                                    <div class="flex items-center gap-4">
                                                        <span class="text-sm font-black uppercase tracking-widest dark:text-white">{day.dia}</span>
                                                        <div class="hairline-divider dark:bg-neutral-800 flex-1"></div>
                                                    </div>
                                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-8">
                                                        {day.comidas.into_iter().map(|meal| view! {
                                                            <div class="flex flex-col gap-1">
                                                                 <span class="text-sm font-medium dark:text-white">{meal.nombre}</span>
                                                                 <span class="text-[10px] text-neutral-400 dark:text-neutral-500 font-bold uppercase tracking-wider">
                                                                     {format!("{} / {}", meal.tipo, meal.ingredientes.join(", "))}
                                                                 </span>
                                                            </div>
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </div>
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                }.into_any()
                            }
                            Some(html) => view! {
                                <article
                                    class="prose-brutalist max-w-none"
                                    inner_html=html
                                />
                            }.into_any(),
                            _ => view! { <div class="py-20 text-center text-neutral-200 uppercase tracking-widest text-[10px]">"No se encontró contenido"</div> }.into_any()
                        }
                    }}
                </Suspense>
            </section>

            // -- PROGRESS METRICS SECTION --
            <section class="px-6 py-12">
                <div class="flex flex-col space-y-6">
                    <div>
                        <div class="flex justify-between items-end mb-2">
                            <span class="text-[10px] font-bold uppercase tracking-widest dark:text-neutral-300">"Integridad de Fibra"</span>
                            <span class="text-[10px] font-bold tabular-nums dark:text-neutral-300">"12g"</span>
                        </div>
                        <div class="w-full h-[1px] bg-neutral-100 dark:bg-neutral-700 relative">
                            <div class="absolute top-0 left-0 h-full bg-neutral-950 dark:bg-white" style="width: 45%;"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between items-end mb-2">
                            <span class="text-[10px] font-bold uppercase tracking-widest dark:text-neutral-300">"Índice de Sodio"</span>
                            <span class="text-[10px] font-bold tabular-nums dark:text-neutral-300">"480mg"</span>
                        </div>
                        <div class="w-full h-[1px] bg-neutral-100 dark:bg-neutral-700 relative">
                            <div class="absolute top-0 left-0 h-full bg-accent" style="width: 20%;"></div>
                        </div>
                    </div>
                </div>
            </section>

            // -- FOOTER ACTIONS --
            <footer class="fixed bottom-0 left-0 right-0 p-6 bg-white/80 dark:bg-background-dark/80 backdrop-blur-lg border-t border-neutral-100 dark:border-neutral-800 z-[45]">
                <button
                    on:click=move |_| set_show_assign_modal.set(true)
                    class="w-full bg-accent py-5 flex items-center justify-center gap-3 active:scale-[0.98] transition-transform text-neutral-950"
                >
                    <span class="text-sm font-bold uppercase tracking-[0.3em]">"Registrar Comida"</span>
                    <span class="material-symbols-outlined !text-base">"add"</span>
                </button>
            </footer>

            // -- OVERLAYS --
            {move || if generating_variation.get() {
                view! {
                    <Portal>
                        <div class="fixed inset-0 bg-white/95 dark:bg-background-dark/95 z-[1000] flex flex-col items-center justify-center animate-in fade-in">
                            <Loading />
                        </div>
                    </Portal>
                }.into_any()
            } else { ().into_any() }}

            {move || if show_assign_modal.get() {
                view! {
                    <Portal>
                        <div class="fixed inset-0 bg-white dark:bg-background-dark z-[500] p-6 flex flex-col pt-24 animate-in fade-in">
                            <button on:click=move |_| set_show_assign_modal.set(false) class="absolute top-6 left-6 w-10 h-10 bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center rounded-full">
                                <span class="material-symbols-outlined">"close"</span>
                            </button>
                            <h2 class="text-4xl font-black uppercase tracking-tighter mb-12 dark:text-white">"Asignar a la Semana"</h2>
                            <div class="flex flex-col gap-4">
                                {vec!["Próximo Lunes", "Próximo Martes", "Selección Manual"].into_iter().map(|label| view! {
                                    <button class="w-full p-6 brutalist-border dark:border-neutral-700 dark:text-white text-left uppercase font-bold text-sm hover:bg-accent dark:hover:bg-accent transition-colors">
                                        {label}
                                    </button>
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </Portal>
                }.into_any()
            } else { ().into_any() }}
        </div>
    }
}
