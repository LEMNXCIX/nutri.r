use crate::models::AppConfig;
use crate::repositories::ConfigRepository;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    state.config_repo.get().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_config(state: State<'_, AppState>, config: AppConfig) -> Result<(), String> {
    state.config_repo.save(&config).map_err(|e| e.to_string())
}
