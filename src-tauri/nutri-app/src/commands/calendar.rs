use nutri_core::models::{CalendarEntry, MealType};
use nutri_core::repositories::ConfigRepository;
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
    recipe_id: Option<String>,
    plan_day_index: Option<u8>,
    assignment_id: Option<String>,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppCalendarService> = state.calendar_service.lock().await;
    service
        .assign_plan(
            date,
            meal_type,
            plan_id,
            recipe_id,
            plan_day_index,
            assignment_id,
        )
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

#[tauri::command]
pub async fn assign_weekly_plan_to_date(
    state: State<'_, AppState>,
    start_date: String,
    plan_id: String,
) -> Result<(), String> {
    let plan_service = state.plan_service.lock().await;
    let detail = plan_service
        .get_plan_detail(&plan_id)
        .map_err(|e| e.to_string())?;
    let index = plan_service.list_plans().map_err(|e| e.to_string())?;
    let plan = index
        .into_iter()
        .find(|p| p.id == plan_id)
        .ok_or_else(|| format!("Plan {} not found", plan_id))?;
    // Drop the lock before getting the next one if it might take time, though here it's fast
    drop(plan_service);

    let config = state.config_repo.get().map_err(|e| e.to_string())?;

    let calendar_service = state.calendar_service.lock().await;
    calendar_service
        .assign_weekly_plan_to_date(
            &start_date,
            &plan_id,
            detail.structured_plan,
            plan.weekly_structure.clone(),
            config.default_meal_type,
        )
        .map_err(|e| e.to_string())?;

    state.trigger_sync().await;
    Ok(())
}

#[tauri::command]
pub async fn swap_calendar_entries(
    state: State<'_, AppState>,
    first_date: String,
    first_meal_type: MealType,
    second_date: String,
    second_meal_type: MealType,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppCalendarService> = state.calendar_service.lock().await;
    service
        .swap_entries(
            &first_date,
            first_meal_type,
            &second_date,
            second_meal_type,
        )
        .map_err(|e: AppError| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}
