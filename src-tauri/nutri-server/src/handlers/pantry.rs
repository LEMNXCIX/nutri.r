use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;
use nutri_core::state::AppState;
use crate::error::ApiError;

// Placeholder
pub async fn list_pantry_items(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, ApiError> {
    Ok(Json(vec![]))
}
