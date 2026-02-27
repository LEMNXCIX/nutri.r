use axum::{
    extract::{State},
    Json,
};
use nutri_core::state::AppState;
use std::sync::Arc;
use crate::error::ApiError;
use serde::Serialize;

#[derive(Serialize)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: String,
    pub size: i64,
}

pub async fn list_models(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<OllamaModel>>, ApiError> {
    // In a real scenario, we would call the Ollama API here
    Ok(Json(vec![]))
}
