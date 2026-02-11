use nutri_core::models::{CalendarEntry, MealType};
use nutri_core::state::{AppCalendarService, AppState};
use nutri_core::utils::AppError;
use tauri::State;
use tokio::sync::MutexGuard;

#[tauri::command]
pub async fn assign_plan_to_date(
    state: State<'_, AppState>,
    date: String,
    meal_type: MealType,
    plan_id: String,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppCalendarService> = state.calendar_service.lock().await;
    service
        .assign_plan(date, meal_type, plan_id)
        .map_err(|e: AppError| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}

#[tauri::command]
pub async fn get_calendar_range(
    state: State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<CalendarEntry>, String> {
    let service: MutexGuard<'_, AppCalendarService> = state.calendar_service.lock().await;
    service
        .get_range(&start_date, &end_date)
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
pub async fn remove_calendar_entry(
    state: State<'_, AppState>,
    date: String,
    meal_type: MealType,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppCalendarService> = state.calendar_service.lock().await;
    service
        .remove_entry(&date, meal_type)
        .map_err(|e: AppError| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}

