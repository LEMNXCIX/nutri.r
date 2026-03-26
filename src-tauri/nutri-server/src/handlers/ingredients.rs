use crate::error::ApiError;
use axum::{extract::State, Json};
use nutri_core::{services::IngredientStats, state::AppState};
use serde::Deserialize;
use std::sync::Arc;

pub async fn get_excluded_ingredients(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, ApiError> {
    let service = state.ingredient_service.lock().await;
    let ingredients = service.get_excluded()?;
    Ok(Json(ingredients))
}

pub async fn get_ingredient_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<IngredientStats>>, ApiError> {
    let service = state.ingredient_service.lock().await;
    let stats = service.get_statistics()?;
    Ok(Json(stats))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleExclusionRequest {
    pub ingredient: String,
}

pub async fn toggle_ingredient_exclusion(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ToggleExclusionRequest>,
) -> Result<Json<Vec<String>>, ApiError> {
    let service = state.ingredient_service.lock().await;
    let ingredients = service.toggle_exclusion(&req.ingredient)?;
    Ok(Json(ingredients))
}

pub async fn save_excluded_ingredients(
    State(state): State<Arc<AppState>>,
    Json(ingredients): Json<Vec<String>>,
) -> Result<Json<()>, ApiError> {
    let service = state.ingredient_service.lock().await;
    service.save_excluded(ingredients)?;
    Ok(Json(()))
}
