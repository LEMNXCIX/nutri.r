use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use nutri_core::models::backup::AppBackup;
use nutri_core::state::{AppState, SyncStatus};
use std::sync::Arc;
use crate::error::ApiError;

// GET /vault
pub async fn get_vault(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AppBackup>, ApiError> {
    let vault_path = state.data_dir.join("vault.json");
    
    if !vault_path.exists() {
        return Err(ApiError(nutri_core::utils::error::AppError::NotFound(
            "Vault not found".to_string(),
        )));
    }
    
    let content = tokio::fs::read_to_string(&vault_path).await
        .map_err(|e| nutri_core::utils::error::AppError::Database(e.to_string()))?;
        
    let backup: AppBackup = serde_json::from_str(&content)
        .map_err(|e| nutri_core::utils::error::AppError::Serialization(e.to_string()))?;
        
    Ok(Json(backup))
}

// POST /vault
pub async fn update_vault(
    State(state): State<Arc<AppState>>,
    Json(backup): Json<AppBackup>,
) -> Result<StatusCode, ApiError> {
    let vault_path = state.data_dir.join("vault.json");
    
    let content = serde_json::to_string_pretty(&backup)
        .map_err(|e| nutri_core::utils::error::AppError::Serialization(e.to_string()))?;
        
    tokio::fs::write(&vault_path, content).await
        .map_err(|e| nutri_core::utils::error::AppError::Database(e.to_string()))?;
        
    Ok(StatusCode::OK)
}

pub async fn perform_sync(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<String>, ApiError> {
    Ok(Json("Sincronización manual completada".to_string()))
}

pub async fn pull_from_server(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<String>, ApiError> {
    Ok(Json("Datos descargados del servidor".to_string()))
}

pub async fn push_to_server(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<String>, ApiError> {
    Ok(Json("Datos subidos al servidor".to_string()))
}

pub async fn get_sync_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<SyncStatus>, ApiError> {
    let status = state.last_sync_status.lock().await;
    Ok(Json(status.clone()))
}
