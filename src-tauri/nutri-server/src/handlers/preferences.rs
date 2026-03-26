use crate::error::ApiError;
use axum::{extract::State, Json};
use nutri_core::{
    models::preferences::UIPreferences, repositories::PreferencesRepository, state::AppState,
};
use std::sync::Arc;

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
