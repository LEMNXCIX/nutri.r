use crate::models::UIPreferences;
use crate::repositories::PreferencesRepository;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_ui_preferences(state: State<'_, AppState>) -> Result<UIPreferences, String> {
    let repo = &state.preferences_repo;
    repo.get().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_ui_preferences(
    state: State<'_, AppState>,
    preferences: UIPreferences,
) -> Result<(), String> {
    let repo = &state.preferences_repo;
    repo.save(&preferences).map_err(|e| e.to_string())
}
