use crate::models::AppBackup;
use crate::repositories::ConfigRepository;
use crate::state::AppState;
use chrono::{DateTime, Local, Utc};
use tauri::State;

#[tauri::command]
pub async fn perform_sync(state: State<'_, AppState>) -> Result<String, String> {
    let mut config = state.config_repo.get().map_err(|e| e.to_string())?;

    if config.sync_server_url.is_empty() {
        return Err("No se ha configurado la URL del servidor de sincronización".to_string());
    }

    let server_url = format!("{}/vault", config.sync_server_url.trim_end_matches('/'));
    let client = reqwest::Client::new();

    // 1. Crear backup local actual
    let mut local_backup = {
        let service = state.import_export_service.lock().await;
        service.create_backup().map_err(|e| e.to_string())?
    };

    // Si nunca se ha sincronizado, ponemos la fecha actual como punto de partida local si no hay una
    if local_backup.last_updated.is_empty() {
        local_backup.last_updated = Local::now().to_rfc3339();
    }

    // 2. Intentar obtener el vault remoto
    let remote_response = client.get(&server_url).send().await;

    match remote_response {
        Ok(resp) if resp.status().is_success() => {
            let remote_backup: AppBackup = resp
                .json()
                .await
                .map_err(|e| format!("Error parseando backup remoto: {}", e))?;

            let local_ts: DateTime<Utc> = DateTime::parse_from_rfc3339(&local_backup.last_updated)
                .map_err(|_| "Invalid local timestamp")?
                .with_timezone(&Utc);
            let remote_ts: DateTime<Utc> =
                DateTime::parse_from_rfc3339(&remote_backup.last_updated)
                    .map_err(|_| "Invalid remote timestamp")?
                    .with_timezone(&Utc);

            if remote_ts > local_ts {
                // EL REMOTO GANA
                {
                    let service = state.import_export_service.lock().await;
                    service
                        .restore_backup(remote_backup.clone())
                        .map_err(|e| e.to_string())?;
                }

                // Actualizar last_updated local
                config.last_updated = remote_backup.last_updated;
                state.config_repo.save(&config).map_err(|e| e.to_string())?;

                Ok("Sincronización completada: Datos remotos importados (LWW)".to_string())
            } else if local_ts > remote_ts {
                // EL LOCAL GANA - Subir al servidor
                let post_resp = client
                    .post(&server_url)
                    .json(&local_backup)
                    .send()
                    .await
                    .map_err(|e| format!("Error subiendo al servidor: {}", e))?;

                if post_resp.status().is_success() {
                    // Actualizar last_updated local con lo que acabamos de enviar
                    config.last_updated = local_backup.last_updated.clone();
                    state.config_repo.save(&config).map_err(|e| e.to_string())?;
                    Ok(
                        "Sincronización completada: Datos locales subidos al servidor (LWW)"
                            .to_string(),
                    )
                } else {
                    Err(format!(
                        "El servidor rechazó el backup: {}",
                        post_resp.status()
                    ))
                }
            } else {
                Ok("Ya estás sincronizado (mismo timestamp)".to_string())
            }
        }
        Ok(resp) if resp.status() == 404 || resp.status() == 204 => {
            // Servidor vacío, subir local
            let post_resp = client
                .post(&server_url)
                .json(&local_backup)
                .send()
                .await
                .map_err(|e| format!("Error inicializando servidor: {}", e))?;

            if post_resp.status().is_success() {
                config.last_updated = local_backup.last_updated;
                state.config_repo.save(&config).map_err(|e| e.to_string())?;
                Ok("Servidor inicializado con éxito".to_string())
            } else {
                Err(format!(
                    "Error inicializando servidor: {}",
                    post_resp.status()
                ))
            }
        }
        _ => {
            // Falló la conexión o el servidor no responde bien
            Err("No se pudo conectar con el servidor de sincronización. Verifica la URL y que el servidor de sincronización (Node.js) esté iniciado.".to_string())
        }
    }
}
