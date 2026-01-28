use crate::tauri_bridge::{get_config, save_config, AppConfig};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Config() -> impl IntoView {
    let (config, set_config) = signal(AppConfig {
        prompt_maestro: String::new(),
        smtp_user: String::new(),
        smtp_password: String::new(),
    });
    let (loading, set_loading) = signal(true);
    let (status_msg, set_status_msg) = signal(String::new());

    spawn_local(async move {
        let result = get_config().await;
        match result {
            Ok(c) => {
                set_config.set(c);
                set_loading.set(false);
            }
            Err(err) => {
                // Si falla (probablemente no existe), dejamos el default y mostramos el form
                set_status_msg.set(format!("Aviso: {}", err));
                set_loading.set(false);
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

    view! {
        <div class="config-page p-6 max-w-2xl mx-auto">
            <h1 class="text-3xl font-bold mb-6">Configuración</h1>

            {move || loading.get().then(|| view! { <p>Cargando configuración...</p> })}

            <form on:submit=on_submit class="space-y-4">
                <div class="flex flex-col">
                    <label for="prompt" class="font-medium mb-1">Prompt Maestro:</label>
                    <textarea
                        id="prompt"
                        class="bg-gray-800 text-white p-2 rounded border border-gray-700 h-32"
                        on:input=move |ev| set_config.update(|c| c.prompt_maestro = event_target_value(&ev))
                        prop:value=move || config.get().prompt_maestro
                        placeholder="Ej: Genera un plan nutricional semanal enfocado en..."
                    />
                </div>

                <div class="flex flex-col">
                    <label for="smtp_user" class="font-medium mb-1">SMTP Usuario:</label>
                    <input
                        id="smtp_user"
                        type="text"
                        class="bg-gray-800 text-white p-2 rounded border border-gray-700"
                        on:input=move |ev| set_config.update(|c| c.smtp_user = event_target_value(&ev))
                        prop:value=move || config.get().smtp_user
                    />
                </div>

                <div class="flex flex-col">
                    <label for="smtp_pass" class="font-medium mb-1">SMTP Contraseña:</label>
                    <input
                        id="smtp_pass"
                        type="password"
                        class="bg-gray-800 text-white p-2 rounded border border-gray-700"
                        on:input=move |ev| set_config.update(|c| c.smtp_password = event_target_value(&ev))
                        prop:value=move || config.get().smtp_password
                    />
                </div>

                <button
                    type="submit"
                    class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors"
                >
                    Guardar Configuración
                </button>
            </form>

            <div class="mt-4">
               <p class="text-sm italic">{move || status_msg.get()}</p>
            </div>
        </div>
    }
}
