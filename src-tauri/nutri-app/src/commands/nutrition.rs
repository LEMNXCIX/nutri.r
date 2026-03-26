use nutri_core::models::PlanNutrition;
use nutri_core::state::AppState;
use nutri_core::utils::AppResult;
use tauri::State;

#[tauri::command]
pub async fn calculate_nutrition(
    state: State<'_, AppState>,
    plan_id: String,
) -> AppResult<PlanNutrition> {
    let service = state.nutrition_service.lock().await;
    service.get_plan_nutrition(&plan_id).await
}
