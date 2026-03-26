use crate::error::ApiError;
use axum::{
    extract::{Path, State},
    Json,
};
use nutri_core::{models::ShoppingList, state::AppState};
use serde::Deserialize;
use std::sync::Arc;

pub async fn get_shopping_list(
    State(state): State<Arc<AppState>>,
    Path(plan_id): Path<String>,
) -> Result<Json<Option<ShoppingList>>, ApiError> {
    let service = state.shopping_service.lock().await;
    let list = service.get_list(&plan_id)?;
    Ok(Json(list))
}

pub async fn generate_shopping_list(
    State(state): State<Arc<AppState>>,
    Path(plan_id): Path<String>,
) -> Result<Json<ShoppingList>, ApiError> {
    let service = state.shopping_service.lock().await;
    let list = service.generate_list_for_plan(&plan_id).await?;
    Ok(Json(list))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleItemRequest {
    pub item_name: String,
    pub checked: bool,
}

pub async fn toggle_item(
    State(state): State<Arc<AppState>>,
    Path(plan_id): Path<String>,
    Json(req): Json<ToggleItemRequest>,
) -> Result<Json<()>, ApiError> {
    let service = state.shopping_service.lock().await;
    service.toggle_item(&plan_id, &req.item_name, req.checked)?;
    Ok(Json(()))
}
