use log::info;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//
use crate::models::{AppConfig, PlanIndex};

pub mod ai;
pub mod db;
pub mod models;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn build_exclusion_list(index: &Vec<PlanIndex>) -> String {
    // Implementar lógica para construir la lista de exclusión
    // Tomamos las últimas 3 semanas (o menos si hay menos)
    let start = if index.len() >= 3 { index.len() - 3 } else { 0 };
    let recent = &index[start..];
    info!("Recent plans: {:?}", recent);
    let proteins: Vec<String> = recent
        .iter()
        .flat_map(|item| item.proteinas.clone())
        .collect();

    proteins.join(", ")
}

#[tauri::command]
async fn generate_week(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 1. Leer Config y Index
    let data_dir = db::get_data_dir(&app_handle);
    let index_path = data_dir.join("index.json");
    let index = db::read_index(&index_path);

    // 2. Construir lista de exclusión (últimas 3 semanas)
    let exclusion_list = build_exclusion_list(&index);

    // 3. Llamar a ai::generate_plan_with_ollama
    // TODO: Obtener prompt de config o usar default
    let prompt = "Genera un plan nutricional semanal.".to_string();
    let (plan_content, proteins) = ai::generate_plan(prompt, exclusion_list).await?;

    // 4. Guardar resultado en archivo y actualizar index
    let plan_id = db::save_plan(&data_dir, &plan_content);
    db::update_index(&index_path, &plan_id, proteins);

    // 5. Retornar "OK" o el ID generado
    Ok(plan_id)
}

#[tauri::command]
async fn get_plan_content(app_handle: tauri::AppHandle, id: &str) -> Result<String, String> {
    let data_dir = db::get_data_dir(&app_handle);
    let plan_path = data_dir.join(format!("{}.json", id));
    db::get_plan_content(&plan_path)
}

#[tauri::command]
async fn get_index(app_handle: tauri::AppHandle) -> Result<Vec<PlanIndex>, String> {
    let data_dir = db::get_data_dir(&app_handle);
    let index_path = data_dir.join("index.json");
    let index = db::read_index(&index_path);
    Ok(index)
}

#[tauri::command]
async fn get_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    let data_dir = db::get_data_dir(&app_handle);
    let config_path = data_dir.join("config.json");
    db::read_config(&config_path)
}

#[tauri::command]
async fn save_config(app_handle: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    let data_dir = db::get_data_dir(&app_handle);
    let config_path = data_dir.join("config.json");
    db::save_config(&config_path, &config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_week,
            get_index,
            get_plan_content,
            get_config,
            save_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
