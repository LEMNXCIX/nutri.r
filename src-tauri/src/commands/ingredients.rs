use crate::services::IngredientStats;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_ingredient_stats(
    state: State<'_, AppState>,
) -> Result<Vec<IngredientStats>, String> {
    let service = state.ingredient_service.lock().await;
    service.get_statistics().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_excluded_ingredients(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let service = state.ingredient_service.lock().await;
    service.get_excluded().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_excluded_ingredients(
    state: State<'_, AppState>,
    ingredients: Vec<String>,
) -> Result<(), String> {
    let service = state.ingredient_service.lock().await;
    service
        .save_excluded(ingredients)
        .map_err(|e| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}

#[tauri::command]
pub async fn toggle_ingredient_exclusion(
    state: State<'_, AppState>,
    ingredient: String,
) -> Result<Vec<String>, String> {
    let service = state.ingredient_service.lock().await;
    let res = service
        .toggle_exclusion(&ingredient)
        .map_err(|e| e.to_string());
    if res.is_ok() {
        state.trigger_sync().await;
    }
    res
}
