use crate::error::ApiError;
use axum::{
    extract::{Path, State},
    Json,
};
use nutri_core::{
    models::{
        metadata::PlanMetadata, plan::VariationType, search::SearchFilters, PlanDetail, PlanIndex,
        RecipeSuggestion, StructuredRecipe,
    },
    state::AppState,
};
use serde::Deserialize;
use std::sync::Arc;

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

pub async fn get_plan_detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<PlanDetail>, ApiError> {
    let service = state.plan_service.lock().await;
    let detail = service.get_plan_detail(&id)?;
    Ok(Json(detail))
}

pub async fn generate_plan(State(state): State<Arc<AppState>>) -> Result<Json<String>, ApiError> {
    let service = state.plan_service.lock().await;
    let id = service.generate_plan().await?;
    Ok(Json(id))
}

pub async fn delete_plan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<()>, ApiError> {
    let service = state.plan_service.lock().await;
    service.delete_plan(&id)?;
    Ok(Json(()))
}

pub async fn get_favorite_plans(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PlanIndex>>, ApiError> {
    let service = state.search_service.lock().await;
    let plans = service.search(SearchFilters {
        only_favorites: true,
        ..Default::default()
    })?;
    Ok(Json(plans))
}

pub async fn toggle_favorite(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<bool>, ApiError> {
    let service = state.metadata_service.lock().await;
    let is_fav = service.toggle_favorite(id)?;
    Ok(Json(is_fav))
}

pub async fn get_metadata(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<PlanMetadata>, ApiError> {
    let service = state.metadata_service.lock().await;
    let meta = service.get_metadata(id)?;
    Ok(Json(meta))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingRequest {
    pub rating: u8,
}

pub async fn set_rating(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<RatingRequest>,
) -> Result<Json<()>, ApiError> {
    let service = state.metadata_service.lock().await;
    service.set_rating(id, req.rating)?;
    Ok(Json(()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteRequest {
    pub note: String,
}

pub async fn set_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<NoteRequest>,
) -> Result<Json<()>, ApiError> {
    let service = state.metadata_service.lock().await;
    service.set_note(id, req.note)?;
    Ok(Json(()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayNameRequest {
    pub display_name: String,
}

pub async fn set_display_name(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<DisplayNameRequest>,
) -> Result<Json<()>, ApiError> {
    let service = state.metadata_service.lock().await;
    service.set_display_name(id, req.display_name)?;
    Ok(Json(()))
}

pub async fn search_plans(
    State(state): State<Arc<AppState>>,
    Json(filters): Json<SearchFilters>,
) -> Result<Json<Vec<PlanIndex>>, ApiError> {
    let service = state.search_service.lock().await;
    let plans = service.search(filters)?;
    Ok(Json(plans))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariationRequest {
    pub variation: VariationType,
}

pub async fn generate_variation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<VariationRequest>,
) -> Result<Json<String>, ApiError> {
    let service = state.plan_service.lock().await;
    let content = service.generate_variation(&id, req.variation).await?;
    Ok(Json(content))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSuggestionRequest {
    pub prompt: String,
}

pub async fn suggest_recipe_edit(
    State(state): State<Arc<AppState>>,
    Path((plan_id, recipe_id)): Path<(String, String)>,
    Json(req): Json<RecipeSuggestionRequest>,
) -> Result<Json<RecipeSuggestion>, ApiError> {
    let service = state.plan_service.lock().await;
    let suggestion = service
        .suggest_recipe_edit(&plan_id, &recipe_id, &req.prompt)
        .await?;
    Ok(Json(suggestion))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyRecipeEditRequest {
    pub recipe: StructuredRecipe,
}

pub async fn apply_recipe_edit(
    State(state): State<Arc<AppState>>,
    Path((plan_id, recipe_id)): Path<(String, String)>,
    Json(req): Json<ApplyRecipeEditRequest>,
) -> Result<Json<PlanDetail>, ApiError> {
    let service = state.plan_service.lock().await;
    let detail = service.apply_recipe_edit(&plan_id, &recipe_id, req.recipe)?;
    state.trigger_sync().await;
    Ok(Json(detail))
}
