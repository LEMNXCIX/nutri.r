use nutri_core::models::UIPreferences;
use nutri_core::repositories::PreferencesRepository;
use nutri_core::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_ui_preferences(state: State<'_, AppState>) -> Result<UIPreferences, String> {
    let repo = &state.preferences_repo;
    repo.get().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_ui_preferences(
    preferences: UIPreferences,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .preferences_repo
        .save(&preferences)
        .map_err(|e| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}
