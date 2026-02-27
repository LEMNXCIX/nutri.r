use crate::error::ApiError;
use axum::{
    extract::{Query, State},
    Json,
};
use nutri_core::{
    models::{calendar::MealType, CalendarEntry},
    repositories::ConfigRepository,
    state::AppState,
    utils::error::AppError,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeQuery {
    pub start_date: String,
    pub end_date: String,
}

pub async fn get_calendar_range(
    State(state): State<Arc<AppState>>,
    Query(range): Query<RangeQuery>,
) -> Result<Json<Vec<CalendarEntry>>, ApiError> {
    let service = state.calendar_service.lock().await;
    let entries = service.get_range(&range.start_date, &range.end_date)?;
    Ok(Json(entries))
}

pub async fn assign_plan(
    State(state): State<Arc<AppState>>,
    Json(entry): Json<CalendarEntry>,
) -> Result<Json<()>, ApiError> {
    let service = state.calendar_service.lock().await;
    service.assign_plan(entry.date, entry.meal_type, entry.plan_id)?;
    Ok(Json(()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEntryRequest {
    pub date: String,
    pub meal_type: MealType,
}

pub async fn remove_entry(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RemoveEntryRequest>,
) -> Result<Json<()>, ApiError> {
    let service = state.calendar_service.lock().await;
    service.remove_entry(&req.date, req.meal_type)?;
    Ok(Json(()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignWeeklyPlanRequest {
    pub start_date: String,
    pub plan_id: String,
}

pub async fn assign_weekly_plan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AssignWeeklyPlanRequest>,
) -> Result<Json<()>, ApiError> {
    let plan_service = state.plan_service.lock().await;
    let index = plan_service
        .list_plans()
        .map_err(|e| ApiError(AppError::Internal(e.to_string())))?;
    let plan = index
        .into_iter()
        .find(|p| p.id == req.plan_id)
        .ok_or_else(|| {
            ApiError(AppError::Internal(format!(
                "Plan {} not found",
                req.plan_id
            )))
        })?;
    drop(plan_service);

    let config = state
        .config_repo
        .get()
        .map_err(|e| ApiError(AppError::Internal(e.to_string())))?;

    let service = state.calendar_service.lock().await;
    service.assign_weekly_plan_to_date(
        &req.start_date,
        &req.plan_id,
        plan.weekly_structure,
        config.default_meal_type,
    )?;

    // Trigger sync
    state.trigger_sync().await;
    Ok(Json(()))
}
