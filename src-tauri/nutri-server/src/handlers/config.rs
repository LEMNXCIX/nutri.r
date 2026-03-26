use crate::error::ApiError;
use axum::{extract::State, Json};
use nutri_core::{models::AppConfig, repositories::ConfigRepository, state::AppState};
use std::sync::Arc;
use tracing::info;

pub async fn get_config(State(state): State<Arc<AppState>>) -> Result<Json<AppConfig>, ApiError> {
    let config = state.config_repo.get()?;
    Ok(Json(config))
}

pub async fn update_config(
    State(state): State<Arc<AppState>>,
    Json(new_config): Json<AppConfig>,
) -> Result<Json<AppConfig>, ApiError> {
    state.config_repo.save(&new_config)?;
    info!("Configuration updated. Changes will take effect on next server restart or scheduler reload.");
    Ok(Json(new_config))
}
