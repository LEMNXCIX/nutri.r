use crate::models::ShoppingList;
use crate::state::{AppShoppingListService, AppState};
use crate::utils::AppError;
use tauri::State;
use tokio::sync::MutexGuard;

#[tauri::command]
pub async fn generate_shopping_list(
    state: State<'_, AppState>,
    plan_id: String,
) -> Result<ShoppingList, String> {
    let service: MutexGuard<'_, AppShoppingListService> = state.shopping_service.lock().await;
    service
        .generate_list_for_plan(&plan_id)
        .await
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
pub async fn get_shopping_list(
    state: State<'_, AppState>,
    plan_id: String,
) -> Result<Option<ShoppingList>, String> {
    let service: MutexGuard<'_, AppShoppingListService> = state.shopping_service.lock().await;
    service
        .get_list(&plan_id)
        .map_err(|e: AppError| e.to_string())
}

#[tauri::command]
pub async fn toggle_shopping_item(
    state: State<'_, AppState>,
    plan_id: String,
    item_name: String,
    checked: bool,
) -> Result<(), String> {
    let service: MutexGuard<'_, AppShoppingListService> = state.shopping_service.lock().await;
    service
        .toggle_item(&plan_id, &item_name, checked)
        .map_err(|e: AppError| e.to_string())
}
