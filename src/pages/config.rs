use crate::components::ui::{Button, Card, Input, Loading, Toast};
use crate::tauri_bridge::{
    get_config, get_excluded_ingredients, get_ui_preferences, is_mobile, list_ollama_models,
    save_config, save_excluded_ingredients, save_ui_preferences, AppConfig, OllamaModel,
    UIPreferences,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
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
    });
    let (loading, set_loading) = signal(true);
    let (status_msg, set_status_msg) = signal(String::new());
    let (ollama_models, set_ollama_models) = signal(Vec::<OllamaModel>::new());
    let (excluded_ingredients, set_excluded_ingredients) = signal(Vec::<String>::new());
    let (new_ingredient, set_new_ingredient) = signal(String::new());
    let (preferences, set_preferences) = signal(UIPreferences::default());
    let (is_mobile_ui, set_is_mobile_ui) = signal(false);

    spawn_local(async move {
        set_is_mobile_ui.set(is_mobile().await);
    });

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
            Ok(prefs) => set_preferences.set(prefs),
            Err(e) => log::warn!("No se pudieron cargar las preferencias: {}", e),
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

    let h_update_ollama_url =
        Callback::new(move |val: String| set_config.update(|c| c.ollama_url = val));
    let h_update_smtp_host =
        Callback::new(move |val: String| set_config.update(|c| c.smtp_host = val));
    let h_update_smtp_port = Callback::new(move |val: String| {
        if let Ok(port) = val.parse::<u16>() {
            set_config.update(|c| c.smtp_port = port);
        }
    });
    let h_update_smtp_user =
        Callback::new(move |val: String| set_config.update(|c| c.smtp_user = val));
    let h_update_smtp_pass =
        Callback::new(move |val: String| set_config.update(|c| c.smtp_password = val));
    let h_update_smtp_to = Callback::new(move |val: String| set_config.update(|c| c.smtp_to = val));
    let h_update_usda_key =
        Callback::new(move |val: String| set_config.update(|c| c.usda_api_key = val));
    let h_update_sync_url =
        Callback::new(move |val: String| set_config.update(|c| c.sync_server_url = val));

    let h_sync_click = Callback::new(move |_: web_sys::MouseEvent| {
        set_status_msg.set("Iniciando sincronización..".to_string());
        spawn_local(async move {
            match crate::tauri_bridge::perform_sync().await {
                Ok(msg) => {
                    set_status_msg.set(msg);
                    if let Ok(c) = get_config().await {
                        set_config.set(c);
                    }
                }
                Err(e) => set_status_msg.set(format!("Error: {}", e)),
            }
        });
    });

    let h_pull_click = Callback::new(move |_: web_sys::MouseEvent| {
        set_status_msg.set("Descargando datos del servidor...".to_string());
        spawn_local(async move {
            match crate::tauri_bridge::pull_from_server().await {
                Ok(msg) => {
                    set_status_msg.set(msg);
                    if let Ok(c) = get_config().await {
                        set_config.set(c);
                    }
                }
                Err(e) => set_status_msg.set(format!("Error: {}", e)),
            }
        });
    });

    let h_push_click = Callback::new(move |_: web_sys::MouseEvent| {
        set_status_msg.set("Subiendo datos al servidor...".to_string());
        spawn_local(async move {
            match crate::tauri_bridge::push_to_server().await {
                Ok(msg) => {
                    set_status_msg.set(msg);
                    if let Ok(c) = get_config().await {
                        set_config.set(c);
                    }
                }
                Err(e) => set_status_msg.set(format!("Error: {}", e)),
            }
        });
    });

    let h_update_new_ing = Callback::new(move |val: String| set_new_ingredient.set(val));

    let h_add_ingredient = Callback::new(add_ingredient);
    let h_save_click = Callback::new(move |_: web_sys::MouseEvent| {});

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
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-10 animate-in fade-in duration-700 font-sans">
            <header class="mb-12 flex flex-col items-center text-center">
                <span class="text-xs font-black text-gray-400 tracking-widest uppercase mb-3">"SYSTEM PREFERENCES"</span>
                <h2 class="text-5xl font-black text-black tracking-tighter mb-4 leading-none">
                    "CONFIGURACIÓN"
                </h2>
                <div class="h-1 w-12 bg-black rounded-full mb-6"></div>
                <p class="text-gray-500 font-medium max-w-lg">"Personaliza tu experiencia y gestiona tus datos. Todos los cambios se guardan localmente."</p>
            </header>

            {move || if loading.get() {
                 view! { <div class="flex justify-center py-20"><Loading /></div> }.into_any()
            } else {
                 ().into_any()
            }}

            <form on:submit=on_submit class="space-y-8">
                // Appearance Section
                <section>
                    <div class="flex items-center gap-3 mb-4 px-2">
                        <div class="p-2 bg-gray-100 rounded-lg text-black">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" /></svg>
                        </div>
                        <h3 class="text-xl font-black text-black tracking-tight">"APARIENCIA"</h3>
                    </div>
                    <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
                        <div class="space-y-3">
                            <label class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Tema Visual"</label>
                            <select
                                class="w-full bg-gray-50 p-4 rounded-xl border border-gray-200 text-black focus:ring-2 focus:ring-black/5 focus:border-black outline-none transition-all uppercase font-bold text-xs tracking-wider"
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    spawn_local(async move {
                                        set_preferences.update(|p| p.theme = val.clone());
                                        let prefs = preferences.get();

                                        if let Err(e) = save_ui_preferences(prefs).await {
                                            log::error!("Error saving preferences: {}", e);
                                        }

                                        let document = web_sys::window().unwrap().document().unwrap();
                                        let html = document.document_element().unwrap();
                                        if val == "dark" {
                                            let _ = html.class_list().add_1("dark");
                                        } else {
                                            let _ = html.class_list().remove_1("dark");
                                        }
                                    });
                                }
                                prop:value=move || preferences.get().theme
                                disabled=move || is_mobile_ui.get()
                            >
                                <option value="light">"Modo Claro"</option>
                                <option value="dark">"Modo Oscuro"</option>
                            </select>
                        </div>
                    </Card>
                </section>

                // AI Section
                <section>
                    <div class="flex items-center gap-3 mb-4 px-2 mt-8">
                        <div class="p-2 bg-gray-100 rounded-lg text-black">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 21h6l-.75-4M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                        </div>
                        <h3 class="text-xl font-black text-black tracking-tight">"CONFIGURACIÓN IA (OLLAMA)"</h3>
                    </div>
                    <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                            <div class="space-y-3">
                                <label for="ollama_url" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"URL del Servidor"</label>
                                <Input
                                    id="ollama_url"
                                    value=Signal::derive(move || config.get().ollama_url)
                                    on_input=h_update_ollama_url
                                    placeholder="http://127.0.0.1:11434"
                                    class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                    disabled=is_mobile_ui
                                />
                            </div>

                            <div class="space-y-3">
                                <label for="ollama_model" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Modelo Seleccionado"</label>
                                <select
                                    id="ollama_model"
                                    class="w-full bg-gray-50 p-4 rounded-xl border border-gray-200 text-black focus:ring-2 focus:ring-black/5 focus:border-black outline-none transition-all uppercase font-bold text-xs tracking-wider"
                                    on:change=move |ev| set_config.update(|c| c.ollama_model = event_target_value(&ev))
                                    prop:value=move || config.get().ollama_model
                                    disabled=move || is_mobile_ui.get()
                                >
                                    <option value="">Selecciona un modelo</option>
                                    {move || {
                                        ollama_models.get().iter().map(|model| {
                                            let name = model.name.clone();
                                            let name_clone = name.clone();
                                            view! {
                                                <option value=name>{name_clone}</option>
                                            }
                                        }).collect::<Vec<_>>()
                                    }}
                                </select>
                            </div>
                        </div>
                    </Card>
                </section>

                // Prompt Section
                <section>
                    <div class="flex items-center gap-3 mb-4 px-2 mt-8">
                        <div class="p-2 bg-gray-100 rounded-lg text-black">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" /></svg>
                        </div>
                        <h3 class="text-xl font-black text-black tracking-tight">"PROMPT MAESTRO"</h3>
                    </div>
                    <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
                        <div class="space-y-3">
                            <label for="prompt" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Instrucciones del Sistema"</label>
                            <textarea
                                id="prompt"
                                class="w-full bg-gray-50 p-6 rounded-2xl border border-gray-200 text-black h-40 focus:ring-2 focus:ring-black/5 focus:border-black outline-none transition-all text-xs font-medium leading-relaxed"
                                on:input=move |ev| set_config.update(|c| c.prompt_maestro = event_target_value(&ev))
                                prop:value=move || config.get().prompt_maestro
                                placeholder="Ej: Genera un plan nutricional semanal enfocado en..."
                                disabled=move || is_mobile_ui.get()
                            />
                        </div>
                    </Card>
                </section>

                // SMTP Section
                <section>
                    <div class="flex items-center gap-3 mb-4 px-2 mt-8">
                        <div class="p-2 bg-gray-100 rounded-lg text-black">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                        </div>
                        <h3 class="text-xl font-black text-black tracking-tight">"CONFIGURACIÓN SMTP"</h3>
                    </div>
                    <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                            <div class="space-y-3">
                                <label for="smtp_host" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Servidor SMTP"</label>
                                <Input
                                    id="smtp_host"
                                    value=Signal::derive(move || config.get().smtp_host)
                                    on_input=h_update_smtp_host
                                    placeholder="smtp.gmail.com"
                                    class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                    disabled=is_mobile_ui
                                />
                            </div>

                            <div class="space-y-3">
                                <label for="smtp_port" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Puerto"</label>
                                <Input
                                    id="smtp_port"
                                    value=Signal::derive(move || config.get().smtp_port.to_string())
                                    on_input=h_update_smtp_port
                                    placeholder="587"
                                    class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                    disabled=is_mobile_ui
                                />
                            </div>

                            <div class="space-y-3">
                                <label for="smtp_user" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Usuario o Email"</label>
                                <Input
                                    id="smtp_user"
                                    value=Signal::derive(move || config.get().smtp_user)
                                    on_input=h_update_smtp_user
                                    placeholder="tu@email.com"
                                    class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                    disabled=is_mobile_ui
                                />
                            </div>

                            <div class="space-y-3">
                                <label for="smtp_pass" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Contraseña"</label>
                                <Input
                                    id="smtp_pass"
                                    type_="password"
                                    value=Signal::derive(move || config.get().smtp_password)
                                    on_input=h_update_smtp_pass
                                    placeholder="••••••••••••"
                                    class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                    disabled=is_mobile_ui
                                />
                            </div>

                            <div class="md:col-span-2 space-y-3">
                                <label for="smtp_to" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"Enviar Reportes A"</label>
                                <Input
                                    id="smtp_to"
                                    value=Signal::derive(move || config.get().smtp_to)
                                    on_input=h_update_smtp_to
                                    placeholder="ejemplo@email.com"
                                    class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                    disabled=is_mobile_ui
                                />
                            </div>
                        </div>
                    </Card>
                </section>

                // Nutrition USDA Section
                <section>
                    <div class="flex items-center gap-3 mb-4 px-2 mt-8">
                        <div class="p-2 bg-gray-100 rounded-lg text-black">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                        </div>
                        <h3 class="text-xl font-black text-black tracking-tight">"NUTRICIÓN (USDA)"</h3>
                    </div>
                    <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
                        <p class="text-[10px] text-gray-500 font-black uppercase tracking-widest leading-relaxed mb-6">
                            "Enlaza tu cuenta con FoodData Central para obtener perfiles nutricionales detallados."
                            <br/>
                            <a href="https://fdc.nal.usda.gov/api-guide.html" target="_blank" class="inline-block mt-2 text-black hover:text-gray-700 underline underline-offset-4 decoration-2">"Obtener API Key gratuita →"</a>
                        </p>

                        <div class="space-y-3">
                            <label for="usda_key" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"USDA API Key"</label>
                            <Input
                                id="usda_key"
                                type_="password"
                                value=Signal::derive(move || config.get().usda_api_key)
                                on_input=h_update_usda_key
                                placeholder="Introduce tu API Key..."
                                class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                disabled=is_mobile_ui
                            />
                        </div>
                    </Card>
                </section>

                // Sync Section
                <section>
                    <div class="flex items-center gap-3 mb-4 px-2 mt-8">
                        <div class="p-2 bg-gray-100 rounded-lg text-black">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" /></svg>
                        </div>
                        <h3 class="text-xl font-black text-black tracking-tight">"SINCRONIZACIÓN"</h3>
                    </div>
                    <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
                        <p class="text-[10px] text-gray-500 font-black uppercase tracking-widest leading-relaxed mb-6">
                            "Conectate a tu servidor Bun para sincronizar tus datos entre dispositivos (PC y Móvil)."
                        </p>

                        <div class="space-y-6">
                            <div class="space-y-3">
                                <label for="sync_url" class="block text-[10px] font-black uppercase tracking-[0.2em] text-gray-500 pl-1">"URL del Servidor Bun"</label>
                                <Input
                                    id="sync_url"
                                    value=Signal::derive(move || config.get().sync_server_url)
                                    on_input=h_update_sync_url
                                    placeholder="http://192.168.1.50:3000"
                                    class="bg-gray-50 border-gray-200 text-black focus:border-black"
                                />
                            </div>

                            <div class="space-y-4">
                                <p class="text-[10px] text-gray-500 font-black uppercase tracking-widest leading-relaxed">
                                    "Elige cómo sincronizar tus datos:"
                                </p>

                                <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
                                    <Button
                                        on_click=h_pull_click
                                        class="flex flex-col items-center gap-2 py-4 bg-white hover:bg-gray-50 text-black border-2 border-black rounded-xl font-black uppercase text-xs tracking-wide transition-all".to_string()
                                    >
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" /></svg>
                                        "⬇ Pull"
                                        <span class="text-[8px] font-medium normal-case tracking-normal text-gray-500">"Traer del servidor"</span>
                                    </Button>

                                    <Button
                                        on_click=h_sync_click
                                        class="flex flex-col items-center gap-2 py-4 bg-black hover:bg-gray-900 text-white border-none rounded-xl font-black uppercase text-xs tracking-wide transition-all shadow-lg shadow-gray-100".to_string()
                                    >
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg>
                                        "⟳ Auto Sync"
                                        <span class="text-[8px] font-medium normal-case tracking-normal text-gray-300">"LWW automático"</span>
                                    </Button>

                                    <Button
                                        on_click=h_push_click
                                        class="flex flex-col items-center gap-2 py-4 bg-white hover:bg-gray-50 text-black border-2 border-black rounded-xl font-black uppercase text-xs tracking-wide transition-all".to_string()
                                    >
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" /></svg>
                                        "⬆ Push"
                                        <span class="text-[8px] font-medium normal-case tracking-normal text-gray-500">"Subir al servidor"</span>
                                    </Button>
                                </div>

                                <div class="px-2 flex justify-between items-center text-[10px] font-bold text-gray-400 uppercase tracking-widest">
                                    <span>"Último cambio/sincro:"</span>
                                    <span class="text-black">{move || {
                                        let ts = config.get().last_updated;
                                        if ts.is_empty() { "Nunca".to_string() } else { ts }
                                    }}</span>
                                </div>
                            </div>
                        </div>
                    </Card>
                </section>

                <div class="pt-8">
                    <Button
                        on_click=h_save_click
                        class="w-full py-5 rounded-[1.5rem] bg-black hover:bg-black/90 text-white font-black text-sm uppercase tracking-widest shadow-xl shadow-gray-200 border-none transform hover:-translate-y-1 transition-all".to_string()
                    >
                        "Guardar Cambios Globales"
                    </Button>
                </div>
            </form>

            // Ingredients Section
            <section class="mt-16">
                 <div class="flex items-center gap-3 mb-4 px-2">
                    <div class="p-2 bg-gray-100 rounded-lg text-black">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" /></svg>
                    </div>
                    <h3 class="text-xl font-black text-black tracking-tight">"INGREDIENTES EXCLUIDOS"</h3>
                </div>
                <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
                    <p class="text-[10px] text-gray-500 font-black uppercase tracking-widest mb-6">
                        "Estos elementos serán omitidos por la IA al generar nuevos planes."
                    </p>

                    <div class="flex gap-3 mb-6">
                        <Input
                            placeholder="Ej: Maní, Gluten, Lactosa..."
                            value=new_ingredient
                            on_input=h_update_new_ing
                            class="flex-1 bg-gray-50 border-gray-200 text-black focus:border-black"
                            disabled=is_mobile_ui
                        />
                        <Button
                            on_click=h_add_ingredient
                            class="px-8 whitespace-nowrap bg-black hover:bg-gray-900 text-white border-none rounded-xl font-bold uppercase text-xs tracking-wide".to_string()
                            disabled=is_mobile_ui
                        >
                            "Agregar"
                        </Button>
                    </div>

                    <div class="flex flex-wrap gap-2">
                        {move || {
                            let ingredients = excluded_ingredients.get();
                            if ingredients.is_empty() {
                                return view! { <p class="text-[10px] text-gray-400 font-black uppercase tracking-widest py-4 w-full text-center">"No hay exclusiones activas"</p> }.into_any()
                            }
                            ingredients.into_iter().map(|ingredient| {
                                let ing = ingredient.clone();
                                let ing_clone = ing.clone();
                                view! {
                                    <div class="bg-gray-50 px-4 py-2 rounded-lg flex items-center gap-3 border border-gray-200 group hover:border-gray-300 transition-all">
                                        <span class="text-[10px] font-black text-gray-700 uppercase tracking-wider">{ing}</span>
                                        <button
                                            type="button"
                                            class="text-gray-400 hover:text-red-500 transition-colors"
                                            on:click=move |_| remove_ingredient(ing_clone.clone())
                                            disabled=move || is_mobile_ui.get()
                                        >
                                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 18L18 6M6 6l12 12" /></svg>
                                        </button>
                                    </div>
                                }
                            }).collect::<Vec<_>>().into_any()
                        }}
                    </div>
                </Card>
            </section>

            // Backup Section
            <section class="mt-16">
                 <div class="flex items-center gap-3 mb-4 px-2">
                    <div class="p-2 bg-gray-100 rounded-lg text-black">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" /></svg>
                    </div>
                    <h3 class="text-xl font-black text-black tracking-tight">"COPIA DE SEGURIDAD"</h3>
                </div>
                <Card class="p-8 bg-white rounded-[2rem] border border-gray-200 shadow-sm hover:shadow-md transition-shadow relative overflow-hidden">
                    <div class="flex flex-col md:flex-row gap-6 mb-8">
                        <Button
                            on_click=handle_export
                            class="flex-1 bg-white hover:bg-gray-50 text-black border-2 border-black py-4 rounded-xl font-black uppercase tracking-widest text-xs".to_string()
                            disabled=is_mobile_ui
                        >
                            "Exportar Vault (.json)"
                        </Button>

                        <div class="flex-1 relative">
                            <input
                                type="file"
                                id="import-file"
                                class="hidden"
                                on:change=handle_import
                                accept=".json"
                            />
                            <label
                                for="import-file"
                                class="w-full flex h-full items-center justify-center px-4 py-4 bg-black hover:bg-gray-900 text-white rounded-xl cursor-pointer transition-all font-black uppercase text-xs tracking-widest"
                                class:opacity-50=move || is_mobile_ui.get()
                                class:pointer-events-none=move || is_mobile_ui.get()
                            >
                                "Importar Vault (.json)"
                            </label>
                        </div>
                    </div>

                    <div class="bg-gray-50 border border-dashed border-gray-300 p-4 rounded-xl">
                        <p class="text-[10px] text-gray-500 font-bold uppercase tracking-wider leading-relaxed flex gap-3">
                            <svg class="w-4 h-4 shrink-0 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                            "Atención: La importación sobrescribirá todos los datos actuales de la aplicación. Haz una copia de seguridad antes de proceder."
                        </p>
                    </div>
                </Card>
            </section>

            {move || if !status_msg.get().is_empty() {
                let msg = status_msg.get();
                let is_err = msg.to_lowercase().contains("error") || msg.to_lowercase().contains("aviso");
                view! {
                    <Toast
                        message=Signal::derive(move || status_msg.get())
                        on_close=Callback::new(move |_| set_status_msg.set(String::new()))
                        is_error=is_err
                    />
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
