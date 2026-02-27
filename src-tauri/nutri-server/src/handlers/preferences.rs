use axum::{
    extract::{State},
    Json,
};
use nutri_core::{models::preferences::UIPreferences, state::AppState, repositories::PreferencesRepository};
use std::sync::Arc;
use crate::error::ApiError;

pub async fn get_preferences(
    State(state): State<Arc<AppState>>,
) -> Result<Json<UIPreferences>, ApiError> {
    let prefs = state.preferences_repo.get().unwrap_or_default();
    Ok(Json(prefs))
}

pub async fn save_preferences(
    State(state): State<Arc<AppState>>,
    Json(prefs): Json<UIPreferences>,
) -> Result<Json<()>, ApiError> {
    state.preferences_repo.save(&prefs)?;
    Ok(Json(()))
}
