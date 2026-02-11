use axum::{
    extract::{Path, State},
    Json,
};
use nutri_core::{models::PlanIndex, state::AppState};
use std::sync::Arc;
use crate::error::ApiError;

pub async fn list_plans(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PlanIndex>>, ApiError> {
    let service = state.plan_service.lock().await;
    let plans = service.list_plans()?;
    Ok(Json(plans))
}

pub async fn get_plan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<String>, ApiError> {
    let service = state.plan_service.lock().await;
    let content = service.get_plan_content(&id)?;
    Ok(Json(content))
}

pub async fn generate_plan(
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, ApiError> {
    let service = state.plan_service.lock().await;
    let id = service.generate_plan().await?;
    Ok(Json(id))
}
