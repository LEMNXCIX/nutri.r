use axum::{
    extract::{Path, State},
    Json,
};
use nutri_core::{models::nutrition::PlanNutrition, state::AppState};
use std::sync::Arc;
use crate::error::ApiError;

pub async fn calculate_nutrition(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<PlanNutrition>, ApiError> {
    let service = state.nutrition_service.lock().await;
    let nutrition = service.get_plan_nutrition(&id).await?;
    Ok(Json(nutrition))
}
