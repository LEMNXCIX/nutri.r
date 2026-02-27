use axum::{
    extract::{Path, State},
    Json,
};
use nutri_core::{models::tag::Tag, state::AppState};
use std::sync::Arc;
use crate::error::ApiError;
use serde::Deserialize;

pub async fn get_all_tags(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Tag>>, ApiError> {
    let service = state.tag_service.lock().await;
    let tags = service.get_all_tags()?;
    Ok(Json(tags))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagRequest {
    pub name: String,
    pub color: String,
}

pub async fn create_tag(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<Tag>, ApiError> {
    let mut service = state.tag_service.lock().await;
    let tag = service.create_tag(req.name, req.color)?;
    Ok(Json(tag))
}

pub async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Path(tag_id): Path<String>,
) -> Result<Json<()>, ApiError> {
    let service = state.tag_service.lock().await;
    service.delete_tag(tag_id)?;
    Ok(Json(()))
}

pub async fn add_tag_to_plan(
    State(state): State<Arc<AppState>>,
    Path((plan_id, tag_id)): Path<(String, String)>,
) -> Result<Json<()>, ApiError> {
    let service = state.tag_service.lock().await;
    service.add_tag_to_plan(plan_id, tag_id)?;
    Ok(Json(()))
}

pub async fn remove_tag_from_plan(
    State(state): State<Arc<AppState>>,
    Path((plan_id, tag_id)): Path<(String, String)>,
) -> Result<Json<()>, ApiError> {
    let service = state.tag_service.lock().await;
    service.remove_tag_from_plan(plan_id, tag_id)?;
    Ok(Json(()))
}
