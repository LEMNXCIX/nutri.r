use crate::models::achievement::Achievement;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_achievements(state: State<'_, AppState>) -> Result<Vec<Achievement>, String> {
    let service = state.achievement_service.lock().await;
    service.get_achievements().map_err(|e| e.to_string())
}
