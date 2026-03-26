use nutri_core::models::Tag;
use nutri_core::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_all_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    let service = state.tag_service.lock().await;
    service.get_all_tags().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_tag(
    state: State<'_, AppState>,
    name: String,
    color: String,
) -> Result<Tag, String> {
    let service = state.tag_service.lock().await;
    let res = service.create_tag(name, color).map_err(|e| e.to_string());
    if res.is_ok() {
        state.trigger_sync().await;
    }
    res
}

#[tauri::command]
pub async fn delete_tag(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let service = state.tag_service.lock().await;
    service.delete_tag(id).map_err(|e| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}

#[tauri::command]
pub async fn add_tag_to_plan(
    state: State<'_, AppState>,
    plan_id: String,
    tag_id: String,
) -> Result<(), String> {
    let service = state.tag_service.lock().await;
    service
        .add_tag_to_plan(plan_id, tag_id)
        .map_err(|e| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}

#[tauri::command]
pub async fn remove_tag_from_plan(
    state: State<'_, AppState>,
    plan_id: String,
    tag_id: String,
) -> Result<(), String> {
    let service = state.tag_service.lock().await;
    service
        .remove_tag_from_plan(plan_id, tag_id)
        .map_err(|e| e.to_string())?;
    state.trigger_sync().await;
    Ok(())
}
