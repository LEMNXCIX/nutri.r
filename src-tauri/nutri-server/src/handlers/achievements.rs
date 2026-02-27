use axum::{
    extract::{State},
    Json,
};
use nutri_core::{models::achievement::Achievement, state::AppState};
use std::sync::Arc;
use crate::error::ApiError;

pub async fn get_achievements(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Achievement>>, ApiError> {
    let service = state.achievement_service.lock().await;
    let achievements = service.get_achievements()?;
    Ok(Json(achievements))
}
