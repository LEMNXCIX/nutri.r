use crate::models::PlanNutrition;
use crate::state::AppState;
use crate::utils::AppResult;
use tauri::State;

#[tauri::command]
pub async fn calculate_nutrition(
    state: State<'_, AppState>,
    plan_id: String,
) -> AppResult<PlanNutrition> {
    let service = state.nutrition_service.lock().await;
    service.get_plan_nutrition(&plan_id).await
}
