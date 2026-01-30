use crate::models::metadata::PlanMetadata;
use crate::state::{AppMetadataService, AppState};
use crate::utils::AppError;
use tauri::State;
use tokio::sync::MutexGuard;

#[tauri::command]
pub async fn toggle_favorite(state: State<'_, AppState>, plan_id: String) -> Result<bool, String> {
    let service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;
    service
        .toggle_favorite(plan_id)
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
pub async fn set_plan_rating(
    state: State<'_, AppState>,
    plan_id: String,
    rating: u8,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;
    service
        .set_rating(plan_id, rating)
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
pub async fn set_plan_note(
    state: State<'_, AppState>,
    plan_id: String,
    note: String,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;
    service
        .set_note(plan_id, note)
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
pub async fn get_plan_metadata(
    state: State<'_, AppState>,
    plan_id: String,
) -> Result<PlanMetadata, String> {
    let service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;
    service
        .get_metadata(plan_id)
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
pub async fn get_favorites(state: State<'_, AppState>) -> Result<Vec<PlanMetadata>, String> {
    let service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;
    service.get_favorites().map_err(|e: AppError| e.to_string())
}
