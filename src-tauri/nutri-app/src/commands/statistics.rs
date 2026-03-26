use nutri_core::models::{IngredientTrend, Statistics};
use nutri_core::state::AppState;
use nutri_core::utils::AppResult;
use tauri::State;

#[tauri::command]
pub async fn get_statistics(state: State<'_, AppState>) -> AppResult<Statistics> {
    state.statistics_service.get_statistics()
}

#[tauri::command]
pub async fn get_ingredient_trends(state: State<'_, AppState>) -> AppResult<Vec<IngredientTrend>> {
    state.statistics_service.get_ingredient_trends()
}
