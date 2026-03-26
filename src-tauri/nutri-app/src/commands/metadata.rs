use nutri_core::models::metadata::PlanMetadata;
use nutri_core::state::{AppMetadataService, AppState};
use nutri_core::utils::AppError;
use tauri::State;
use tokio::sync::MutexGuard;

#[tauri::command]
pub async fn toggle_favorite(state: State<'_, AppState>, plan_id: String) -> Result<bool, String> {
    let service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;
    let res = service
        .toggle_favorite(plan_id)
        .map_err(|e: AppError| e.to_string());
    if res.is_ok() {
        state.trigger_sync().await;
    }
    res
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
        .map_err(|e: AppError| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
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
        .map_err(|e: AppError| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}

#[tauri::command]
pub async fn set_plan_display_name(
    state: State<'_, AppState>,
    plan_id: String,
    display_name: String,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;
    service
        .set_display_name(plan_id, display_name)
        .map_err(|e: AppError| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
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
