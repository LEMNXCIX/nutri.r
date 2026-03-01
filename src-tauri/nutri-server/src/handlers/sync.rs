use crate::error::ApiError;
use axum::{extract::State, http::StatusCode, Json};
use nutri_core::models::backup::AppBackup;
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
    Json(backup): Json<AppBackup>,
) -> Result<axum::response::Response, ApiError> {
    use axum::response::IntoResponse;

    let vault_path = state.data_dir.join("vault.json");

    let remote_ts = chrono::DateTime::parse_from_rfc3339(&backup.last_updated)
        .map(|dt| dt.timestamp_millis())
        .unwrap_or(0);

    let mut local_ts = 0;

    if vault_path.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&vault_path).await {
            if let Ok(local_data) = serde_json::from_str::<AppBackup>(&content) {
                local_ts = chrono::DateTime::parse_from_rfc3339(&local_data.last_updated)
                    .map(|dt| dt.timestamp_millis())
                    .unwrap_or(0);
            }
        }
    }

    // LWW logic: Guardar solo si remoto >= local
    if remote_ts >= local_ts {
        let content = serde_json::to_string_pretty(&backup)
            .map_err(|e| nutri_core::utils::error::AppError::Serialization(e.to_string()))?;

        tokio::fs::write(&vault_path, content)
            .await
            .map_err(|e| nutri_core::utils::error::AppError::Database(e.to_string()))?;

        let response_body = serde_json::json!({
            "status": "updated",
            "last_updated": backup.last_updated
        });

        Ok((StatusCode::OK, Json(response_body)).into_response())
    } else {
        let response_body = serde_json::json!({
            "status": "ignored",
            "reason": "stale_data",
            "current_ts": chrono::DateTime::from_timestamp_millis(local_ts).map(|dt| dt.to_rfc3339()).unwrap_or_default()
        });

        Ok((StatusCode::CONFLICT, Json(response_body)).into_response())
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
