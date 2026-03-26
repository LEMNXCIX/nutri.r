use crate::error::ApiError;
use axum::{extract::State, Json};
use nutri_core::{models::statistics::Statistics, state::AppState};
use std::sync::Arc;

pub async fn get_statistics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Statistics>, ApiError> {
    let service = &state.statistics_service;
    let stats = service.get_statistics()?;
    Ok(Json(stats))
}

pub async fn get_ingredient_trends(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<nutri_core::models::statistics::IngredientTrend>>, ApiError> {
    let service = &state.statistics_service;
    let trends = service.get_ingredient_trends()?;
    Ok(Json(trends))
}
