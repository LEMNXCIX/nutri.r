pub mod commands;

use nutri_core::state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize AppState with data directory
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            // Ensure data directory exists
            if !data_dir.exists() {
                std::fs::create_dir_all(&data_dir).expect("failed to create data dir");
            }

            let app_state = AppState::new(data_dir);
            let state_for_loop = app_state.clone();
            let app_handle = app.handle().clone();

            // Background sync loop
            tauri::async_runtime::spawn(async move {
                let state = state_for_loop;
                let app = app_handle;

                loop {
                    // Wait for either a trigger or a timeout (5 minutes)
                    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(300));
                    let trigger = state.sync_trigger.notified();

                    tokio::select! {
                        _ = timeout => {
                            let _ = crate::commands::sync::perform_sync_internal(&app, &state).await;
                        }
                        _ = trigger => {
                            // Debounce: wait a bit after the first change to batch multiple changes
                            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                            let _ = crate::commands::sync::perform_sync_internal(&app, &state).await;
                        }
                    }
                }
            });

            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::generate_week,
            commands::get_index,
            commands::get_favorite_plans,
            commands::get_plan_content,
            commands::get_plan_detail,
            commands::get_config,
            commands::is_mobile,
            commands::save_config,
            commands::list_ollama_models,
            commands::get_excluded_ingredients,
            commands::save_excluded_ingredients,
            commands::get_ingredient_stats,
            commands::toggle_ingredient_exclusion,
            commands::metadata::toggle_favorite,
            commands::metadata::get_plan_metadata,
            commands::metadata::get_favorites,
            commands::metadata::set_plan_rating,
            commands::metadata::set_plan_note,
            commands::metadata::set_plan_display_name,
            commands::shopping::generate_shopping_list,
            commands::shopping::get_shopping_list,
            commands::shopping::toggle_shopping_item,
            commands::calendar::assign_plan_to_date,
            commands::calendar::assign_weekly_plan_to_date,
            commands::calendar::get_calendar_range,
            commands::calendar::remove_calendar_entry,
            commands::calendar::swap_calendar_entries,
            commands::statistics::get_statistics,
            commands::statistics::get_ingredient_trends,
            commands::nutrition::calculate_nutrition,
            commands::plans::generate_variation,
            commands::plans::search_plans,
            commands::plans::preview_recipe_edit,
            commands::plans::apply_recipe_edit,
            commands::tags::get_all_tags,
            commands::tags::create_tag,
            commands::tags::delete_tag,
            commands::tags::add_tag_to_plan,
            commands::tags::remove_tag_from_plan,
            commands::pantry::get_pantry_items,
            commands::pantry::add_pantry_item,
            commands::pantry::update_pantry_item,
            commands::pantry::delete_pantry_item,
            commands::export_data,
            commands::import_data,
            commands::preferences::get_ui_preferences,
            commands::preferences::save_ui_preferences,
            commands::achievements::get_achievements,
            commands::email::send_plan_email,
            commands::sync::perform_sync,
            commands::sync::get_sync_status,
            commands::sync::pull_from_server,
            commands::sync::push_to_server,
            commands::water::get_water_intake,
            commands::water::update_water_intake,
            commands::water::get_water_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
