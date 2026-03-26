use nutri_core::models::{
    PlanDetail, PlanIndex, RecipeSuggestion, SearchFilters, StructuredRecipe, VariationType,
};
use nutri_core::state::{AppMetadataService, AppState};
use tauri::State;
use tokio::sync::MutexGuard;

#[tauri::command]
pub async fn generate_week(state: State<'_, AppState>) -> Result<String, String> {
    let service = state.plan_service.lock().await;
    let res = service.generate_plan().await.map_err(|e| e.to_string());
    if res.is_ok() {
        state.trigger_sync().await;
    }
    res
}

#[tauri::command]
pub async fn get_index(state: State<'_, AppState>) -> Result<Vec<PlanIndex>, String> {
    let mut plans = state
        .plan_service
        .lock()
        .await
        .list_plans()
        .map_err(|e| e.to_string())?;
    let metadata_service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;

    for plan in &mut plans {
        if let Ok(meta) = metadata_service.get_metadata(plan.id.clone()) {
            plan.is_favorite = meta.is_favorite;
            plan.rating = meta.rating;
        }
    }

    Ok(plans)
}

#[tauri::command]
pub async fn get_favorite_plans(state: State<'_, AppState>) -> Result<Vec<PlanIndex>, String> {
    let plans = state
        .plan_service
        .lock()
        .await
        .list_plans()
        .map_err(|e| e.to_string())?;
    let metadata_service: MutexGuard<'_, AppMetadataService> = state.metadata_service.lock().await;

    let mut favorites = Vec::new();
    for mut plan in plans {
        if let Ok(meta) = metadata_service.get_metadata(plan.id.clone()) {
            if meta.is_favorite {
                plan.is_favorite = true;
                plan.rating = meta.rating;
                favorites.push(plan);
            }
        }
    }
    Ok(favorites)
}

#[tauri::command]
pub async fn get_plan_content(state: State<'_, AppState>, id: String) -> Result<String, String> {
    let service = state.plan_service.lock().await;
    service.get_plan_content(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_plan_detail(state: State<'_, AppState>, id: String) -> Result<PlanDetail, String> {
    let service = state.plan_service.lock().await;
    service.get_plan_detail(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_variation(
    state: State<'_, AppState>,
    plan_id: String,
    variation: VariationType,
) -> Result<String, String> {
    let service = state.plan_service.lock().await;
    let res = service
        .generate_variation(&plan_id, variation)
        .await
        .map_err(|e| e.to_string());
    if res.is_ok() {
        state.trigger_sync().await;
    }
    res
}

#[tauri::command]
pub async fn search_plans(
    state: State<'_, AppState>,
    filters: SearchFilters,
) -> Result<Vec<PlanIndex>, String> {
    let service = state.search_service.lock().await;
    service.search(filters).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn preview_recipe_edit(
    state: State<'_, AppState>,
    plan_id: String,
    recipe_id: String,
    prompt: String,
) -> Result<RecipeSuggestion, String> {
    let service = state.plan_service.lock().await;
    service
        .suggest_recipe_edit(&plan_id, &recipe_id, &prompt)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn apply_recipe_edit(
    state: State<'_, AppState>,
    plan_id: String,
    recipe_id: String,
    recipe: StructuredRecipe,
) -> Result<PlanDetail, String> {
    let service = state.plan_service.lock().await;
    let result = service
        .apply_recipe_edit(&plan_id, &recipe_id, recipe)
        .map_err(|e| e.to_string());
    if result.is_ok() {
        state.trigger_sync().await;
    }
    result
}
