use axum::{
    extract::{Path, State},
    Json,
};
use nutri_core::{models::pantry::PantryItem, state::AppState};
use std::sync::Arc;
use crate::error::ApiError;

pub async fn list_pantry_items(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PantryItem>>, ApiError> {
    let service = state.pantry_service.lock().await;
    let items = service.get_all_items()?;
    Ok(Json(items))
}

pub async fn add_pantry_item(
    State(state): State<Arc<AppState>>,
    Json(item): Json<PantryItem>,
) -> Result<Json<()>, ApiError> {
    let service = state.pantry_service.lock().await;
    service.add_item(item)?;
    Ok(Json(()))
}

pub async fn update_pantry_item(
    State(state): State<Arc<AppState>>,
    Json(item): Json<PantryItem>,
) -> Result<Json<()>, ApiError> {
    let service = state.pantry_service.lock().await;
    service.update_item(item)?;
    Ok(Json(()))
}

pub async fn delete_pantry_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<()>, ApiError> {
    let service = state.pantry_service.lock().await;
    service.delete_item(&id)?;
    Ok(Json(()))
}
