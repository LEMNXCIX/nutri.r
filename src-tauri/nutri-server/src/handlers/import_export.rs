use crate::error::ApiError;
use axum::{extract::State, Json};
use nutri_core::{models::backup::AppBackup, state::AppState};
use std::sync::Arc;

pub async fn export_data(State(state): State<Arc<AppState>>) -> Result<Json<AppBackup>, ApiError> {
    let service = state.import_export_service.lock().await;
    let backup = service.create_backup()?;
    Ok(Json(backup))
}

pub async fn import_data(
    State(state): State<Arc<AppState>>,
    Json(backup): Json<AppBackup>,
) -> Result<Json<()>, ApiError> {
    let service = state.import_export_service.lock().await;
    service.restore_backup(backup)?;
    Ok(Json(()))
}
