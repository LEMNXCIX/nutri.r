use chrono::{Local, Utc};
use nutri_core::models::AppBackup;
use nutri_core::repositories::ConfigRepository;
use nutri_core::services::{SyncAction, SyncService};
use nutri_core::state::{AppState, SyncStatus};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

#[derive(Debug, Clone, Serialize)]
pub struct SyncStatusPayload {
    pub status: SyncStatus,
}

async fn update_sync_status(app: &AppHandle, state: &AppState, status: SyncStatus) {
    let mut last_status = state.last_sync_status.lock().await;
    *last_status = status.clone();
    let _ = app.emit("sync-status-changed", SyncStatusPayload { status });
}

#[tauri::command]
pub async fn get_sync_status(state: State<'_, AppState>) -> Result<SyncStatus, String> {
    let status = state.last_sync_status.lock().await;
    Ok(status.clone())
}

#[tauri::command]
pub async fn perform_sync(app: AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    perform_sync_internal(&app, &state).await
}

pub async fn perform_sync_internal(app: &AppHandle, state: &AppState) -> Result<String, String> {
    update_sync_status(app, state, SyncStatus::Syncing).await;

    let res = (|| async {
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

        // Si hay una modificación local pendiente, esa es nuestra marca de tiempo real
        let last_mod = state.last_modified.lock().await;
        if let Some(ts) = &*last_mod {
            local_backup.last_updated = ts.clone();
        }

        if local_backup.last_updated.is_empty() {
            local_backup.last_updated = Utc::now().to_rfc3339();
        }

        // 2. Intentar obtener el vault remoto
        let remote_response = client.get(&server_url).send().await;

        match remote_response {
            Ok(resp) if resp.status().is_success() => {
                let remote_backup: AppBackup = resp
                    .json()
                    .await
                    .map_err(|e| format!("Error parseando backup remoto: {}", e))?;

                let (action, reason) = SyncService::resolve_conflict(&local_backup, &remote_backup);

                match action {
                    SyncAction::PullRemote => {
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

                        let _ = app.emit("data-changed", ());

                        Ok(format!("Sincronización (PULL): {}", reason))
                    }
                    SyncAction::PushLocal => {
                        // EL LOCAL GANA - Subir al servidor
                        let post_resp = client
                            .post(&server_url)
                            .json(&local_backup)
                            .send()
                            .await
                            .map_err(|e| format!("Error subiendo al servidor: {}", e))?;

                        if post_resp.status().is_success() {
                            config.last_updated = local_backup.last_updated.clone();
                            state.config_repo.save(&config).map_err(|e| e.to_string())?;
                            Ok(format!("Sincronización (PUSH): {}", reason))
                        } else {
                            Err(format!("El servidor rechazó el backup: {}", post_resp.status()))
                        }
                    }
                    SyncAction::NoAction => Ok(reason),
                    SyncAction::Conflict => Err("Conflicto detectado. Se requiere intervención manual.".to_string()),
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
                    Err(format!("Error inicializando servidor: {}", post_resp.status()))
                }
            }
            _ => Err("No se pudo conectar con el servidor de sincronización.".to_string()),
        }
    })().await;

    match &res {
        Ok(_) => {
            let mut last_mod = state.last_modified.lock().await;
            *last_mod = None;
            update_sync_status(app, state, SyncStatus::Success).await;
        }
        Err(e) => update_sync_status(app, state, SyncStatus::Error(e.clone())).await,
    }

    res
}

#[tauri::command]
pub async fn pull_from_server(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let mut config = state.config_repo.get().map_err(|e| e.to_string())?;

    if config.sync_server_url.is_empty() {
        return Err("No se ha configurado la URL del servidor de sincronización".to_string());
    }

    let server_url = format!("{}/vault", config.sync_server_url.trim_end_matches('/'));
    let client = reqwest::Client::new();

    // Obtener datos del servidor
    let remote_response = client.get(&server_url).send().await;

    match remote_response {
        Ok(resp) if resp.status().is_success() => {
            let remote_backup: AppBackup = resp
                .json()
                .await
                .map_err(|e| format!("Error parseando backup remoto: {}", e))?;

            // Forzar restauración sin importar timestamps
            {
                let service = state.import_export_service.lock().await;
                service
                    .restore_backup(remote_backup.clone())
                    .map_err(|e| e.to_string())?;
            }

            // Actualizar last_updated local
            config.last_updated = remote_backup.last_updated;
            state.config_repo.save(&config).map_err(|e| e.to_string())?;

            // Emitir evento de cambio de datos para que el frontend recargue
            let _ = state.sync_trigger.notify_one();
            let _ = app.emit("data-changed", ());

            Ok("Pull completado: Datos del servidor importados".to_string())
        }
        Ok(resp) if resp.status() == 404 || resp.status() == 204 => {
            Err("El servidor no tiene datos para descargar".to_string())
        }
        _ => Err("No se pudo conectar con el servidor de sincronización".to_string()),
    }
}

#[tauri::command]
pub async fn push_to_server(state: State<'_, AppState>) -> Result<String, String> {
    let mut config = state.config_repo.get().map_err(|e| e.to_string())?;

    if config.sync_server_url.is_empty() {
        return Err("No se ha configurado la URL del servidor de sincronización".to_string());
    }

    let server_url = format!("{}/vault", config.sync_server_url.trim_end_matches('/'));
    let client = reqwest::Client::new();

    // Crear backup local actual
    let mut local_backup = {
        let service = state.import_export_service.lock().await;
        service.create_backup().map_err(|e| e.to_string())?
    };

    // Asegurar que tiene timestamp
    if local_backup.last_updated.is_empty() {
        local_backup.last_updated = Local::now().to_rfc3339();
    }

    // Forzar subida sin importar timestamps
    let post_resp = client
        .post(&server_url)
        .json(&local_backup)
        .send()
        .await
        .map_err(|e| format!("Error subiendo al servidor: {}", e))?;

    if post_resp.status().is_success() {
        config.last_updated = local_backup.last_updated.clone();
        state.config_repo.save(&config).map_err(|e| e.to_string())?;
        Ok("Push completado: Datos locales subidos al servidor".to_string())
    } else {
        Err(format!(
            "El servidor rechazó el backup: {}",
            post_resp.status()
        ))
    }
}
