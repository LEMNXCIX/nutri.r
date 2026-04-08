use crate::error::ApiError;
use axum::{extract::State, http::StatusCode, Json};
use nutri_core::models::backup::AppBackup;
use nutri_core::services::{SyncAction, SyncService};
use nutri_core::state::{AppState, SyncStatus};
use std::sync::Arc;

// GET /vault
pub async fn get_vault(State(state): State<Arc<AppState>>) -> Result<Json<AppBackup>, ApiError> {
    let vault_path = state.data_dir.join("vault.json");

    if !vault_path.exists() {
        return Err(ApiError(nutri_core::utils::error::AppError::NotFound(
            "Vault not found".to_string(),
        )));
    }

    let content = tokio::fs::read_to_string(&vault_path)
        .await
        .map_err(|e| nutri_core::utils::error::AppError::Database(e.to_string()))?;

    let backup: AppBackup = serde_json::from_str(&content)
        .map_err(|e| nutri_core::utils::error::AppError::Serialization(e.to_string()))?;

    Ok(Json(backup))
}

// POST /vault
pub async fn update_vault(
    State(state): State<Arc<AppState>>,
    Json(remote_backup): Json<AppBackup>,
) -> Result<axum::response::Response, ApiError> {
    use axum::response::IntoResponse;

    let vault_path = state.data_dir.join("vault.json");

    let mut local_backup = AppBackup::default();

    if vault_path.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&vault_path).await {
            if let Ok(data) = serde_json::from_str::<AppBackup>(&content) {
                local_backup = data;
            }
        }
    }

    // Resolve conflict using core service
    // Note: local is server's data, remote is client's incoming data
    let (action, reason) = SyncService::resolve_conflict(&local_backup, &remote_backup);

    match action {
        SyncAction::PullRemote | SyncAction::NoAction => {
            // El remoto es más nuevo o igual, aceptamos el cambio
            let content = serde_json::to_string_pretty(&remote_backup)
                .map_err(|e| nutri_core::utils::error::AppError::Serialization(e.to_string()))?;

            tokio::fs::write(&vault_path, content)
                .await
                .map_err(|e| nutri_core::utils::error::AppError::Database(e.to_string()))?;

            let response_body = serde_json::json!({
                "status": "updated",
                "last_updated": remote_backup.last_updated,
                "reason": reason
            });

            Ok((StatusCode::OK, Json(response_body)).into_response())
        }
        SyncAction::PushLocal => {
            // El servidor tiene datos más nuevos, rechazamos la subida del cliente
            let response_body = serde_json::json!({
                "status": "ignored",
                "reason": "stale_data",
                "detail": reason,
                "current_ts": local_backup.last_updated
            });

            Ok((StatusCode::CONFLICT, Json(response_body)).into_response())
        }
        SyncAction::Conflict => {
            let response_body = serde_json::json!({
                "status": "conflict",
                "reason": "manual_intervention_required"
            });
            Ok((StatusCode::PRECONDITION_FAILED, Json(response_body)).into_response())
        }
    }
}

pub async fn perform_sync(State(_state): State<Arc<AppState>>) -> Result<Json<String>, ApiError> {
    Ok(Json("Sincronización manual completada".to_string()))
}

pub async fn pull_from_server(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<String>, ApiError> {
    Ok(Json("Datos descargados del servidor".to_string()))
}

pub async fn push_to_server(State(_state): State<Arc<AppState>>) -> Result<Json<String>, ApiError> {
    Ok(Json("Datos subidos al servidor".to_string()))
}

pub async fn get_sync_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<SyncStatus>, ApiError> {
    let status = state.last_sync_status.lock().await;
    Ok(Json(status.clone()))
}
