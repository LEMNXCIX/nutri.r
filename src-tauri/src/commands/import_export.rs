use crate::models::AppBackup;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn export_data(state: State<'_, AppState>) -> Result<AppBackup, String> {
    let service = state.import_export_service.lock().await;
    service.create_backup().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_data(state: State<'_, AppState>, backup: AppBackup) -> Result<(), String> {
    let service = state.import_export_service.lock().await;
    service.restore_backup(backup).map_err(|e| e.to_string())
}
