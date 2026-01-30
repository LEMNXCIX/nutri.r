use crate::models::PantryItem;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_pantry_items(state: State<'_, AppState>) -> Result<Vec<PantryItem>, String> {
    let service = state.pantry_service.lock().await;
    service.get_all_items().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_pantry_item(state: State<'_, AppState>, item: PantryItem) -> Result<(), String> {
    let service = state.pantry_service.lock().await;
    service.add_item(item).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_pantry_item(
    state: State<'_, AppState>,
    item: PantryItem,
) -> Result<(), String> {
    let service = state.pantry_service.lock().await;
    service.update_item(item).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_pantry_item(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let service = state.pantry_service.lock().await;
    service.delete_item(&id).map_err(|e| e.to_string())
}
