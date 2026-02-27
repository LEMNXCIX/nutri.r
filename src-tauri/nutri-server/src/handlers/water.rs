use axum::{
    extract::{Path, State, Query},
    Json,
};
use nutri_core::{models::WaterRecord, state::AppState, utils::error::AppError};
use std::sync::Arc;
use crate::error::ApiError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type WaterData = HashMap<String, WaterRecord>;

fn get_water_file_path(state: &AppState) -> PathBuf {
    state.data_dir.join("water.json")
}

fn load_water_data(state: &AppState) -> WaterData {
    let path = get_water_file_path(state);
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_water_data(state: &AppState, data: &WaterData) -> Result<(), ApiError> {
    let path = get_water_file_path(state);
    let content = serde_json::to_string_pretty(data).map_err(|e| ApiError(AppError::Serialization(e.to_string())))?;
    fs::write(&path, content).map_err(|e| ApiError(AppError::Database(e.to_string())))?;
    Ok(())
}

pub async fn get_water_intake(
    State(state): State<Arc<AppState>>,
    Path(date): Path<String>,
) -> Result<Json<WaterRecord>, ApiError> {
    let data = load_water_data(&state);
    let record = data.get(&date).cloned().unwrap_or_else(|| WaterRecord {
        current: 0.0,
        target: 2.5,
        last_updated: chrono::Utc::now().to_rfc3339(),
    });
    Ok(Json(record))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWaterRequest {
    pub current: f32,
    pub target: f32,
}

pub async fn update_water_intake(
    State(state): State<Arc<AppState>>,
    Path(date): Path<String>,
    Json(req): Json<UpdateWaterRequest>,
) -> Result<Json<()>, ApiError> {
    let mut data = load_water_data(&state);
    
    data.insert(
        date,
        WaterRecord {
            current: req.current,
            target: req.target,
            last_updated: chrono::Utc::now().to_rfc3339(),
        },
    );
    
    save_water_data(&state, &data)?;
    state.trigger_sync().await;
    
    Ok(Json(()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryQuery {
    pub start_date: String,
    pub end_date: String,
}

pub async fn get_water_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<HashMap<String, WaterRecord>>, ApiError> {
    let data = load_water_data(&state);
    let history = data.into_iter()
        .filter(|(date, _)| date >= &query.start_date && date <= &query.end_date)
        .collect();
    Ok(Json(history))
}
