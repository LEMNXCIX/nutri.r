use crate::tauri_bridge::{
    check_endpoint, clear_debug_logs, get_config, get_debug_logs, get_excluded_ingredients,
    get_ui_preferences, is_mobile, list_ollama_models, save_config, save_excluded_ingredients,
    save_ui_preferences, AppConfig, OllamaModel, UIPreferences,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn Config() -> impl IntoView {
    let (config, set_config) = signal(AppConfig {
        smtp_host: String::new(),
        smtp_port: 0,
        smtp_user: String::new(),
        smtp_password: String::new(),
        smtp_to: String::new(),
        prompt_maestro: String::new(),
        ollama_model: String::new(),
        ollama_url: String::new(),
        usda_api_key: String::new(),
        sync_server_url: String::new(),
        last_updated: String::new(),
        auto_generate_plan: false,
        cron_expression: "0 0 0 * * MON".to_string(),
        default_meal_type: crate::tauri_bridge::MealType::Lunch,
    });
    let (_loading, set_loading) = signal(true);
    let (status_msg, set_status_msg) = signal(String::new());
    let (ollama_models, set_ollama_models) = signal(Vec::<OllamaModel>::new());
    let (excluded_ingredients, set_excluded_ingredients) = signal(Vec::<String>::new());
    let (new_ingredient, set_new_ingredient) = signal(String::new());
    let (preferences, set_preferences) = signal(UIPreferences::default());
    let (_is_mobile_ui, set_is_mobile_ui) = signal(false);
    let (ollama_status_msg, set_ollama_status_msg) = signal(String::new());
    let (sync_status_msg, set_sync_status_msg) = signal(String::new());
    let (sync_op_status, set_sync_op_status) = signal(Option::<(bool, String, String)>::None); // (is_ok, title, detail)
    let (is_syncing, set_is_syncing) = signal(Option::<&'static str>::None); // which op is running
    let (debug_logs, set_debug_logs) = signal(get_debug_logs());

    // Listen for debug logs
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let cb = Closure::wrap(Box::new(move |ev: web_sys::CustomEvent| {
            if let Some(msg) = ev.detail().as_string() {
                set_debug_logs.update(|logs| {
                    logs.push(msg);
                    if logs.len() > 100 {
                        logs.remove(0);
                    }
                });
            }
        }) as Box<dyn FnMut(web_sys::CustomEvent)>);

        let _ = window.add_event_listener_with_callback("debug-log", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    spawn_local(async move {
        set_is_mobile_ui.set(is_mobile().await);
    });

    log::info!("Rendering Config page");

    spawn_local(async move {
        match get_config().await {
            Ok(c) => {
                set_config.set(c);
                set_loading.set(false);
            }
            Err(err) => {
                set_status_msg.set(format!("Aviso: {}", err));
                set_loading.set(false);
            }
        }
    });

    spawn_local(async move {
        match list_ollama_models().await {
            Ok(models) => set_ollama_models.set(models),
            Err(e) => log::warn!("No se pudieron cargar los modelos de Ollama: {}", e),
        }
    });

    spawn_local(async move {
        match get_excluded_ingredients().await {
            Ok(ingredients) => set_excluded_ingredients.set(ingredients),
            Err(e) => log::warn!("No se pudieron cargar los ingredientes excluidos: {}", e),
        }
    });

    spawn_local(async move {
        match get_ui_preferences().await {
            Ok(mut prefs) => {
                let resolved_theme = crate::theme::resolved_theme(Some(&prefs.theme));
                let should_sync_preferences = prefs.theme != resolved_theme;

                prefs.theme = resolved_theme.clone();
                set_preferences.set(prefs.clone());
                crate::theme::set_theme(&resolved_theme);

                if should_sync_preferences {
                    let _ = save_ui_preferences(prefs).await;
                }
            }
            Err(e) => {
                let fallback_theme = crate::theme::resolved_theme(None);
                set_preferences.set(UIPreferences {
                    theme: fallback_theme.clone(),
                    primary_color: "green".to_string(),
                });
                crate::theme::set_theme(&fallback_theme);
                log::warn!("No se pudieron cargar las preferencias: {}", e);
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let current_config = config.get();
        spawn_local(async move {
            match save_config(current_config).await {
                Ok(_) => set_status_msg.set("Configuración guardada correctamente.".to_string()),
                Err(e) => set_status_msg.set(format!("Error al guardar: {}", e)),
            }
        });
    };

    let add_ingredient = move |_| {
        let ingredient = new_ingredient.get().trim().to_string();
        if !ingredient.is_empty() {
            set_excluded_ingredients.update(|list| {
                if !list.contains(&ingredient) {
                    list.push(ingredient.clone());
                }
            });
            set_new_ingredient.set(String::new());

            let ingredients = excluded_ingredients.get();
            spawn_local(async move {
                if let Err(e) = save_excluded_ingredients(ingredients).await {
                    log::error!("Error al guardar ingredientes excluidos: {}", e);
                }
            });
        }
    };

    let remove_ingredient = move |ingredient: String| {
        set_excluded_ingredients.update(|list| {
            list.retain(|i| i != &ingredient);
        });

        let ingredients = excluded_ingredients.get();
        spawn_local(async move {
            if let Err(e) = save_excluded_ingredients(ingredients).await {
                log::error!("Error al guardar ingredientes excluidos: {}", e);
            }
        });
    };

    let _h_update_ollama_url =
        Callback::new(move |val: String| set_config.update(|c| c.ollama_url = val));
    let _h_update_smtp_host =
        Callback::new(move |val: String| set_config.update(|c| c.smtp_host = val));
    let _h_update_smtp_port = Callback::new(move |val: String| {
        if let Ok(port) = val.parse::<u16>() {
            set_config.update(|c| c.smtp_port = port);
        }
    });
    let _h_update_smtp_user =
        Callback::new(move |val: String| set_config.update(|c| c.smtp_user = val));
    let _h_update_smtp_pass =
        Callback::new(move |val: String| set_config.update(|c| c.smtp_password = val));
    let _h_update_smtp_to =
        Callback::new(move |val: String| set_config.update(|c| c.smtp_to = val));
    let _h_update_usda_key =
        Callback::new(move |val: String| set_config.update(|c| c.usda_api_key = val));
    let _h_update_sync_url =
        Callback::new(move |val: String| set_config.update(|c| c.sync_server_url = val));

    let h_sync_click = Callback::new(move |_: web_sys::MouseEvent| {
        set_is_syncing.set(Some("sync"));
        set_sync_op_status.set(None);
        spawn_local(async move {
            match crate::tauri_bridge::perform_sync().await {
                Ok(msg) => {
                    set_sync_op_status.set(Some((
                        true,
                        "Sincronización completada".to_string(),
                        msg,
                    )));
                    if let Ok(c) = get_config().await {
                        set_config.set(c);
                    }
                }
                Err(e) => {
                    set_sync_op_status.set(Some((false, "Error al sincronizar".to_string(), e)))
                }
            }
            set_is_syncing.set(None);
        });
    });

    let h_pull_click = Callback::new(move |_: web_sys::MouseEvent| {
        set_is_syncing.set(Some("pull"));
        set_sync_op_status.set(None);
        spawn_local(async move {
            match crate::tauri_bridge::pull_from_server().await {
                Ok(msg) => {
                    set_sync_op_status.set(Some((true, "Datos recibidos".to_string(), msg)));
                    if let Ok(c) = get_config().await {
                        set_config.set(c);
                    }
                }
                Err(e) => {
                    set_sync_op_status.set(Some((false, "Error al recibir datos".to_string(), e)))
                }
            }
            set_is_syncing.set(None);
        });
    });

    let h_push_click = Callback::new(move |_: web_sys::MouseEvent| {
        set_is_syncing.set(Some("push"));
        set_sync_op_status.set(None);
        spawn_local(async move {
            match crate::tauri_bridge::push_to_server().await {
                Ok(msg) => {
                    set_sync_op_status.set(Some((true, "Datos enviados".to_string(), msg)));
                    if let Ok(c) = get_config().await {
                        set_config.set(c);
                    }
                }
                Err(e) => {
                    set_sync_op_status.set(Some((false, "Error al enviar datos".to_string(), e)))
                }
            }
            set_is_syncing.set(None);
        });
    });

    let _h_update_new_ing = Callback::new(move |val: String| set_new_ingredient.set(val));

    let _h_add_ingredient = Callback::new(add_ingredient);
    let _h_save_click = Callback::new(move |_: web_sys::MouseEvent| {});

    let handle_export = Callback::new(move |_: web_sys::MouseEvent| {
        spawn_local(async move {
            match crate::tauri_bridge::export_data().await {
                Ok(backup) => {
                    if let Ok(json) = serde_json::to_string_pretty(&backup) {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let element = document.create_element("a").unwrap();
                        let blob =
                            web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&json.into()))
                                .unwrap();
                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

                        let a = element.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                        a.set_href(&url);
                        a.set_download("nutri_r_backup.json");
                        a.click();
                        web_sys::Url::revoke_object_url(&url).unwrap();

                        set_status_msg.set("Copia de seguridad generada con éxito.".to_string());
                    }
                }
                Err(e) => set_status_msg.set(format!("Error al exportar: {}", e)),
            }
        });
    });

    let handle_import = move |ev: leptos::ev::Event| {
        let input = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                spawn_local(async move {
                    let promise = file.text();
                    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                    if let Ok(js_val) = result {
                        if let Some(json_content) = js_val.as_string() {
                            match serde_json::from_str::<crate::tauri_bridge::AppBackup>(
                                &json_content,
                            ) {
                                Ok(backup) => {
                                    match crate::tauri_bridge::import_data(backup).await {
                                        Ok(_) => set_status_msg.set(
                                            "Datos importados con éxito. Reinicia la aplicación."
                                                .to_string(),
                                        ),
                                        Err(e) => {
                                            set_status_msg.set(format!("Error al importar: {}", e))
                                        }
                                    }
                                }
                                Err(e) => set_status_msg
                                    .set(format!("Error al procesar el archivo: {}", e)),
                            }
                        }
                    }
                });
            }
        }
    };

    view! {
        <div class="pb-32">
            <main class="px-6 pt-10">
                <h1 class="text-7xl font-extrabold uppercase leading-[0.85] tracking-tighter mb-12 dark:text-white">
                    "AJUSTES"
                </h1>

                <form on:submit=on_submit>
                    {/* Appearance Section */}
                    <ConfigSection title="Apariencia" icon="palette">
                        <div class="flex flex-col gap-4">
                            <div class="flex flex-col gap-1.5">
                                <label class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Tema Visual del Sistema"</label>
                                <select
                                    class="w-full bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                    on:change=move |ev| {
                                        let theme = crate::theme::set_theme(&event_target_value(&ev));
                                        set_preferences.update(|p| p.theme = theme.clone());
                                        let prefs = preferences.get();
                                        spawn_local(async move {
                                            let _ = save_ui_preferences(prefs).await;
                                        });
                                    }
                                    prop:value=move || preferences.get().theme
                                >
                                    <option value="light">"MODO CLARO (EDITORIAL)"</option>
                                    <option value="dark">"MODO OSCURO (BRUTALIST)"</option>
                                </select>
                            </div>
                        </div>
                    </ConfigSection>

                    {/* Local Intelligence Section */}
                    <ConfigSection title="Inteligencia Local" icon="memory">
                        <div class="grid grid-cols-1 gap-6">
                            <div class="flex flex-col gap-1.5">
                                <label class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Endpoint del Servidor"</label>
                                <div class="flex gap-2">
                                    <input
                                        class="flex-1 bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="text"
                                        placeholder="http://localhost:11434"
                                        prop:value=move || config.get().ollama_url
                                        on:input=move |ev| {
                                            set_config.update(|c| c.ollama_url = event_target_value(&ev));
                                            set_ollama_status_msg.set(String::new());
                                        }
                                    />
                                    <button
                                        class="px-4 py-2 bg-black dark:bg-neutral-800 text-white dark:text-neutral-200 text-[10px] font-bold uppercase hover:bg-neutral-800 dark:hover:bg-neutral-700 border border-black dark:border-neutral-600 transition-colors"
                                        on:click=move |_| {
                                            let url = config.get().ollama_url.clone();
                                            spawn_local(async move {
                                                set_ollama_status_msg.set("Verificando...".to_string());
                                                match check_endpoint(&url).await {
                                                    Ok(_) => set_ollama_status_msg.set("✅ Conexión exitosa".to_string()),
                                                    Err(e) => set_ollama_status_msg.set(format!("❌ {}", e)),
                                                }
                                            });
                                        }
                                    >
                                        "PROBAR"
                                    </button>
                                </div>
                                {move || {
                                    let msg = ollama_status_msg.get();
                                    if !msg.is_empty() {
                                        let color_class = if msg.contains("✅") { "text-green-600 dark:text-green-400" } else { "text-red-600 dark:text-red-400" };
                                        view! {
                                            <div class=format!("text-[10px] font-bold uppercase mt-1 {}", color_class)>
                                                {msg}
                                            </div>
                                        }.into_any()
                                    } else {
                                        ().into_any()
                                    }
                                }}
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <label class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Modelo de Lenguaje"</label>
                                <select
                                    class="w-full bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                    on:change=move |ev| set_config.update(|c| c.ollama_model = event_target_value(&ev))
                                    prop:value=move || config.get().ollama_model
                                >
                                    <option value="">"SELECCIONAR MODELO..."</option>
                                    {move || {
                                        let models = ollama_models.get();
                                        models.into_iter().map(|model| {
                                            let name_val = model.name.clone();
                                            let name_display = model.name.clone().to_uppercase();
                                            view! { <option value=name_val>{name_display}</option> }
                                        }).collect_view()
                                    }}
                                </select>
                            </div>
                        </div>
                    </ConfigSection>

                    {/* Master Prompt Section */}
                    <ConfigSection title="Prompt Maestro" icon="edit_note">
                        <div class="space-y-6">
                            <div class="flex flex-col gap-1.5">
                                <label class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Arquitectura del Plan"</label>
                                <textarea
                                    class="w-full h-32 text-[11px] leading-relaxed bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                    placeholder="El plan debe ser práctico, variado (especialmente en fuentes de proteína para aprovechar disponibilidad local), eficiente (máximo 2-3 horas de preparación el domingo) y minimizar el tiempo diario (5-10 minutos de recalentado/montaje)."
                                    on:input=move |ev| set_config.update(|c| c.prompt_maestro = event_target_value(&ev))
                                    prop:value=move || config.get().prompt_maestro
                                />
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <label class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Comida por Defecto"</label>
                                <select
                                    class="w-full bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                    on:change=move |ev| {
                                        let val = event_target_value(&ev);
                                        let meal = match val.as_str() {
                                            "Breakfast" => crate::tauri_bridge::MealType::Breakfast,
                                            "Dinner" => crate::tauri_bridge::MealType::Dinner,
                                            "Snack" => crate::tauri_bridge::MealType::Snack,
                                            _ => crate::tauri_bridge::MealType::Lunch,
                                        };
                                        set_config.update(|c| c.default_meal_type = meal);
                                    }
                                    prop:value=move || {
                                        match config.get().default_meal_type {
                                            crate::tauri_bridge::MealType::Breakfast => "Breakfast",
                                            crate::tauri_bridge::MealType::Lunch => "Lunch",
                                            crate::tauri_bridge::MealType::Dinner => "Dinner",
                                            crate::tauri_bridge::MealType::Snack => "Snack",
                                        }
                                    }
                                >
                                    <option value="Breakfast">"DESAYUNO"</option>
                                    <option value="Lunch">"ALMUERZO"</option>
                                    <option value="Dinner">"CENA"</option>
                                    <option value="Snack">"SNACK"</option>
                                </select>
                            </div>
                        </div>
                    </ConfigSection>

                    {/* Automation Section */}
                    <ConfigSection title="Automatización" icon="schedule">
                        <div class="space-y-6">
                            <div class="flex items-center justify-between py-2">
                                <span class="text-[11px] font-bold uppercase tracking-wider dark:text-white">"Generación Automática"</span>
                                <button
                                    type="button"
                                    on:click=move |_| set_config.update(|c| c.auto_generate_plan = !c.auto_generate_plan)
                                    class="w-12 h-6 bg-black dark:bg-neutral-700 relative flex items-center px-1"
                                >
                                    <div class=move || format!("w-4 h-4 transition-all {}",
                                        if config.get().auto_generate_plan { "bg-accent translate-x-6" } else { "bg-neutral-600 translate-x-0" }
                                    )></div>
                                </button>
                            </div>
                            <div class="flex flex-col gap-1.5">
                                <label class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Frecuencia (Cron)"</label>
                                <div class="flex">
                                    <input
                                        class="flex-grow bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="text"
                                        prop:value=move || config.get().cron_expression
                                        on:input=move |ev| set_config.update(|c| c.cron_expression = event_target_value(&ev))
                                    />
                                    <div class="bg-black dark:bg-neutral-700 text-white px-3 flex items-center text-[10px] font-bold">
                                        {move || {
                                            let cron = config.get().cron_expression;
                                            if cron.contains("MON") { "LUNES" }
                                            else if cron.contains("SUN") { "DOMINGO" }
                                            else { "PERSONALIZADO" }
                                        }}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </ConfigSection>

                    {/* Synchronization Section */}
                    <ConfigSection title="Sincronización" icon="sync">
                        <div class="space-y-6">
                            <div class="flex flex-col gap-1.5">
                                <label class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500">"Endpoint de Sincronía"</label>
                                <div class="flex gap-2">
                                    <input
                                        class="flex-1 bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="text"
                                        placeholder="http://localhost:3001/api/sync"
                                        prop:value=move || config.get().sync_server_url
                                        on:input=move |ev| {
                                            set_config.update(|c| c.sync_server_url = event_target_value(&ev));
                                            set_sync_status_msg.set(String::new());
                                        }
                                    />
                                    <button
                                        class="px-4 py-2 bg-black dark:bg-neutral-800 text-white dark:text-neutral-200 text-[10px] font-bold uppercase hover:bg-neutral-800 dark:hover:bg-neutral-700 border border-black dark:border-neutral-600 transition-colors"
                                        on:click=move |_| {
                                            let url = config.get().sync_server_url.clone();
                                            spawn_local(async move {
                                                set_sync_status_msg.set("Verificando...".to_string());
                                                match check_endpoint(&url).await {
                                                    Ok(_) => set_sync_status_msg.set("✅ Servidor activo".to_string()),
                                                    Err(e) => set_sync_status_msg.set(format!("❌ {}", e)),
                                                }
                                            });
                                        }
                                    >
                                        "PROBAR"
                                    </button>
                                </div>
                                {move || {
                                    let msg = sync_status_msg.get();
                                    if !msg.is_empty() {
                                        let color_class = if msg.contains("✅") { "text-green-600 dark:text-green-400" } else { "text-red-600 dark:text-red-400" };
                                        view! {
                                            <div class=format!("text-[10px] font-bold uppercase mt-1 {}", color_class)>
                                                {msg}
                                            </div>
                                        }.into_any()
                                    } else {
                                        ().into_any()
                                    }
                                }}
                            </div>

                        // Sync Operation Buttons
                        <div class="grid grid-cols-3 gap-2">
                            // PULL: receive from server
                            <button
                                type="button"
                                on:click=move |e| h_pull_click.run(e)
                                disabled=move || is_syncing.get().is_some()
                                class="border border-black dark:border-neutral-700 dark:text-white p-4 flex flex-col items-center gap-1.5 hover:bg-neutral-50 dark:hover:bg-neutral-900 active:bg-neutral-100 transition-colors disabled:opacity-50 group"
                            >
                                {move || if is_syncing.get() == Some("pull") {
                                    view! { <span class="material-symbols-outlined animate-spin">"sync"</span> }.into_any()
                                } else {
                                    view! { <span class="material-symbols-outlined">"cloud_download"</span> }.into_any()
                                }}
                                <span class="text-[9px] font-bold uppercase tracking-wider">"Recibir"</span>
                                <span class="text-[8px] text-neutral-400 dark:text-neutral-500 uppercase">"Servidor → App"</span>
                            </button>

                            // SYNC: bidirectional smart sync
                            <button
                                type="button"
                                on:click=move |e| h_sync_click.run(e)
                                disabled=move || is_syncing.get().is_some()
                                class="bg-black dark:bg-white text-white dark:text-black p-4 flex flex-col items-center gap-1.5 hover:bg-neutral-800 dark:hover:bg-neutral-100 active:bg-neutral-900 transition-colors disabled:opacity-50"
                            >
                                {move || if is_syncing.get() == Some("sync") {
                                    view! { <span class="material-symbols-outlined animate-spin text-accent">"sync"</span> }.into_any()
                                } else {
                                    view! { <span class="material-symbols-outlined text-accent">"sync"</span> }.into_any()
                                }}
                                <span class="text-[9px] font-bold uppercase tracking-wider">"Sincronizar"</span>
                                <span class="text-[8px] uppercase opacity-60">"Inteligente"</span>
                            </button>

                            // PUSH: send to server
                            <button
                                type="button"
                                on:click=move |e| h_push_click.run(e)
                                disabled=move || is_syncing.get().is_some()
                                class="border border-black dark:border-neutral-700 dark:text-white p-4 flex flex-col items-center gap-1.5 hover:bg-neutral-50 dark:hover:bg-neutral-900 active:bg-neutral-100 transition-colors disabled:opacity-50 group"
                            >
                                {move || if is_syncing.get() == Some("push") {
                                    view! { <span class="material-symbols-outlined animate-spin">"sync"</span> }.into_any()
                                } else {
                                    view! { <span class="material-symbols-outlined">"cloud_upload"</span> }.into_any()
                                }}
                                <span class="text-[9px] font-bold uppercase tracking-wider">"Enviar"</span>
                                <span class="text-[8px] text-neutral-400 dark:text-neutral-500 uppercase">"App → Servidor"</span>
                            </button>
                        </div>

                        // Inline sync result panel
                        {move || match sync_op_status.get() {
                            Some((is_ok, title, detail)) => {
                                let (bg, border, icon, text_color) = if is_ok {
                                    ("bg-green-50 dark:bg-green-950/40", "border-green-500", "check_circle", "text-green-700 dark:text-green-400")
                                } else {
                                    ("bg-red-50 dark:bg-red-950/40", "border-red-500", "error", "text-red-700 dark:text-red-400")
                                };
                                view! {
                                    <div class=format!("border-l-4 {} {} p-4 flex items-start gap-3", border, bg)>
                                        <span class=format!("material-symbols-outlined {} flex-shrink-0 !text-xl", text_color)>{icon}</span>
                                        <div class="flex-1 min-w-0">
                                            <p class=format!("text-[10px] font-black uppercase tracking-widest {}", text_color)>{title}</p>
                                            <p class="text-[10px] text-neutral-600 dark:text-neutral-400 mt-1 leading-relaxed break-words">{detail}</p>
                                        </div>
                                        <button
                                            type="button"
                                            on:click=move |_| set_sync_op_status.set(None)
                                            class="material-symbols-outlined !text-sm text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-200 flex-shrink-0"
                                        >
                                            "close"
                                        </button>
                                    </div>
                                }.into_any()
                            }
                            None if is_syncing.get().is_some() => {
                                let label = match is_syncing.get() {
                                    Some("pull") => "Descargando datos del servidor...",
                                    Some("push") => "Subiendo datos al servidor...",
                                    _ => "Sincronizando de forma inteligente...",
                                };
                                view! {
                                    <div class="border border-black dark:border-neutral-700 p-4 flex items-center gap-3">
                                        <span class="material-symbols-outlined animate-spin !text-xl">"sync"</span>
                                        <p class="text-[10px] font-black uppercase tracking-widest">{label}</p>
                                    </div>
                                }.into_any()
                            }
                            _ => ().into_any()
                        }}
                        </div>
                    </ConfigSection>

                    {/* SMTP & Nutrition Sections */}
                    <section class="mb-10">
                        <div class="grid grid-cols-1 gap-12">
                            <div>
                                <div class="section-header">
                                    <span class="material-symbols-outlined">"mail"</span>
                                    <h2 class="section-title">"SMTP"</h2>
                                </div>
                                <div class="grid grid-cols-2 gap-2">
                                    <input
                                        class="col-span-2 bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="text"
                                        placeholder="smtp.gmail.com"
                                        prop:value=move || config.get().smtp_host
                                        on:input=move |ev| set_config.update(|c| c.smtp_host = event_target_value(&ev))
                                    />
                                    <input
                                        class="bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="text"
                                        placeholder="587"
                                        prop:value=move || config.get().smtp_port.to_string()
                                        on:input=move |ev| {
                                            if let Ok(port) = event_target_value(&ev).parse::<u16>() {
                                                set_config.update(|c| c.smtp_port = port);
                                            }
                                        }
                                    />
                                    <input
                                        class="bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="password"
                                        placeholder="**********"
                                        prop:value=move || config.get().smtp_password
                                        on:input=move |ev| set_config.update(|c| c.smtp_password = event_target_value(&ev))
                                    />
                                    <input
                                        class="col-span-2 bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="text"
                                        placeholder="user@mail.com"
                                        prop:value=move || config.get().smtp_user
                                        on:input=move |ev| set_config.update(|c| c.smtp_user = event_target_value(&ev))
                                    />
                                    <input
                                        class="col-span-2 bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="text"
                                        placeholder="target@mail.com"
                                        prop:value=move || config.get().smtp_to
                                        on:input=move |ev| set_config.update(|c| c.smtp_to = event_target_value(&ev))
                                    />
                                </div>
                            </div>
                            <div>
                                <div class="section-header">
                                    <span class="material-symbols-outlined">"nutrition"</span>
                                    <h2 class="section-title">"Nutrition USDA"</h2>
                                </div>
                                <div class="flex flex-col gap-4">
                                    <input
                                        class="bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                        type="password"
                                        prop:value=move || config.get().usda_api_key
                                        on:input=move |ev| set_config.update(|c| c.usda_api_key = event_target_value(&ev))
                                    />
                                    <a href="https://fdc.nal.usda.gov/api-guide.html" target="_blank" class="text-[9px] font-bold uppercase underline text-left tracking-widest dark:text-neutral-400 dark:hover:text-white transition-colors">
                                        "Obtener API KEY —"
                                    </a>
                                </div>
                            </div>
                        </div>
                        <div class="hairline-divider mt-10"></div>
                    </section>

                    {/* Constraints Section */}
                    <ConfigSection title="Restricciones" icon="block">
                        <div class="space-y-4">
                            <div class="flex gap-2">
                                <input
                                    class="flex-grow bg-white dark:bg-neutral-900 border-black dark:border-neutral-700 text-black dark:text-white"
                                    placeholder="Ingresa ingrediente..."
                                    type="text"
                                    prop:value=new_ingredient
                                    on:input=move |ev| set_new_ingredient.set(event_target_value(&ev))
                                    on:keydown=move |ev| {
                                        if ev.key() == "Enter" {
                                            ev.prevent_default();
                                            add_ingredient(());
                                        }
                                    }
                                />
                                <button
                                    type="button"
                                    on:click=move |_| add_ingredient(())
                                    class="bg-black dark:bg-neutral-800 text-white px-4 text-[10px] font-bold uppercase"
                                >
                                    "Añadir"
                                </button>
                            </div>
                            <div class="flex flex-wrap gap-2">
                                {move || {
                                    let ingredients = excluded_ingredients.get();
                                    ingredients.into_iter().map(|ingredient| {
                                        let ing = ingredient.clone();
                                        view! {
                                            <div class="border border-black dark:border-neutral-700 px-2 py-1 flex items-center gap-2">
                                                <span class="text-[10px] font-bold uppercase dark:text-white">{ing.clone()}</span>
                                                <button
                                                    type="button"
                                                    class="material-symbols-outlined text-xs dark:text-neutral-400 hover:text-red-500 dark:hover:text-red-400"
                                                    on:click={let ing = ing.clone(); move |_| remove_ingredient(ing.clone())}
                                                >
                                                    "close"
                                                </button>
                                            </div>
                                        }
                                    }).collect_view()
                                }}
                            </div>
                        </div>
                    </ConfigSection>

                    {/* Data Vault Section */}
                    <section class="mb-20">
                        <div class="section-header">
                            <span class="material-symbols-outlined dark:text-white">"database"</span>
                            <h2 class="section-title dark:text-white">"Data Vault"</h2>
                        </div>
                        <div class="grid grid-cols-2 gap-4">
                            <button
                                type="button"
                                on:click=move |e| handle_export.run(e)
                                class="border border-black dark:border-neutral-700 dark:text-white py-4 text-[10px] font-bold uppercase tracking-widest hover:bg-neutral-50 dark:hover:bg-neutral-900 active:bg-neutral-100 dark:active:bg-neutral-800 transition-colors"
                            >
                                "Exportar .JSON"
                            </button>
                            <label
                                for="import-file"
                                class="bg-black dark:bg-white text-white dark:text-black py-4 text-[10px] font-bold uppercase tracking-widest text-center cursor-pointer hover:bg-neutral-900 dark:hover:bg-neutral-100 active:bg-neutral-800 dark:active:bg-neutral-200 transition-colors"
                            >
                                "Importar .JSON"
                                <input type="file" id="import-file" class="hidden" on:change=handle_import accept=".json" />
                            </label>
                        </div>
                    </section>

                    <button
                        type="submit"
                        class="w-full bg-black dark:bg-white text-white dark:text-black py-6 mb-12 flex items-center justify-center gap-4 hover:bg-neutral-900 dark:hover:bg-neutral-100 active:scale-[0.98] transition-transform"
                    >
                        <span class="text-xs font-bold uppercase tracking-[0.3em]">"Persistir Cambios Globales"</span>
                        <span class="material-symbols-outlined text-accent">"bolt"</span>
                    </button>

                    {/* Debug Logger Section */}
                    {move || {
                        let logs = debug_logs.get();
                        if !logs.is_empty() {
                            view! {
                                <ConfigSection title="Consola de Depuración" icon="terminal">
                                    <div class="flex flex-col gap-4">
                                        <div class="w-full h-48 bg-black dark:bg-neutral-900 border border-neutral-700 p-2 overflow-y-auto font-mono text-[10px] text-green-500 dark:text-green-400">
                                            {logs.into_iter().rev().map(|log| {
                                                view! { <div class="mb-1 border-b border-neutral-800 pb-1">{log}</div> }
                                            }).collect_view()}
                                        </div>
                                        <button
                                            type="button"
                                            on:click=move |_| {
                                                clear_debug_logs();
                                                set_debug_logs.set(Vec::new());
                                            }
                                            class="w-full border border-black dark:border-neutral-700 dark:text-white py-2 text-[9px] font-bold uppercase hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-colors"
                                        >
                                            "Limpiar Consola"
                                        </button>
                                    </div>
                                </ConfigSection>
                            }.into_any()
                        } else {
                            ().into_any()
                        }
                    }}
                </form>
            </main>

            {move || if !status_msg.get().is_empty() {
                let msg = status_msg.get();
                let is_err = msg.to_lowercase().contains("error") || msg.to_lowercase().contains("aviso");
                view! {
                    <div class=format!("fixed top-24 left-1/2 -translate-x-1/2 z-[100] px-6 py-3 border border-black shadow-brutalist flex items-center gap-3 animate-in slide-in-from-top duration-300 {}",
                        if is_err { "bg-red-50" } else { "bg-accent/10" }
                    )>
                        <span class="text-[10px] font-black uppercase tracking-widest">
                            {move || status_msg.get()}
                        </span>
                        <button on:click=move |_| set_status_msg.set(String::new())>
                            <span class="material-symbols-outlined text-xs">"close"</span>
                        </button>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}

#[component]
fn ConfigSection(title: &'static str, icon: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="mb-10">
            <div class="section-header">
                <span class="material-symbols-outlined dark:text-neutral-300">{icon}</span>
                <h2 class="section-title dark:text-white">{title}</h2>
            </div>
            {children()}
            <div class="hairline-divider mt-10 dark:bg-neutral-800"></div>
        </section>
    }
}

#[component]
fn SyncButton(
    label: &'static str,
    on_click: Callback<web_sys::MouseEvent>,
    icon: &'static str,
    #[prop(default = false)] primary: bool,
) -> impl IntoView {
    let base_class = if primary {
        "bg-black dark:bg-white text-white dark:text-black p-4 flex flex-col items-center gap-1 active:bg-neutral-800 dark:active:bg-neutral-200 transition-colors"
    } else {
        "border border-black dark:border-neutral-700 dark:text-white p-4 flex flex-col items-center gap-1 group active:bg-neutral-50 dark:active:bg-neutral-900 transition-colors"
    };

    let icon_class = if primary {
        "material-symbols-outlined text-accent"
    } else {
        "material-symbols-outlined"
    };

    view! {
        <button type="button" on:click=move |e| on_click.run(e) class=base_class>
            <span class=icon_class>{icon}</span>
            <span class="text-[9px] font-bold uppercase">{label}</span>
        </button>
    }
}
