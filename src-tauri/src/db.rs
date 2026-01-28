use crate::models::{AppConfig, PlanDetail, PlanIndex};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

pub fn get_data_dir(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory")
}
pub fn read_index(path: &PathBuf) -> Vec<PlanIndex> {
    if !path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(path).expect("Failed to read index file");
    serde_json::from_str(&content).expect("Failed to parse index file")
}

pub fn read_config(path: &PathBuf) -> Result<AppConfig, String> {
    if !path.exists() {
        return Err("Configuration file not found".to_string());
    }
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
}

pub fn save_config(path: &PathBuf, config: &AppConfig) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }
    let content =
        serde_json::to_string(config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))
}

pub fn write_index(path: &PathBuf, index: &Vec<PlanIndex>) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create data directory");
    }
    let content = serde_json::to_string(index).expect("Failed to serialize index");
    fs::write(path, content).expect("Failed to write index file");
}

pub fn save_plan(data_dir: &PathBuf, content: &str) -> String {
    fs::create_dir_all(data_dir).expect("Failed to create data directory");

    let id = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("{}.json", id);
    let path = data_dir.join(filename);

    let plan_detail = PlanDetail {
        markdown_content: content.to_string(),
    };

    let json = serde_json::to_string(&plan_detail).expect("Failed to serialize plan detail");
    fs::write(path, json).expect("Failed to write plan file");

    id
}

pub fn update_index(path: &PathBuf, id: &str, proteins: Vec<String>) {
    let mut index = read_index(path);

    let new_entry = PlanIndex {
        id: id.to_string(),
        fecha: chrono::Local::now().format("%Y-%m-%d").to_string(),
        proteinas: proteins,
        enviado: false,
    };

    index.push(new_entry);
    write_index(path, &index);
}

pub fn get_plan_content(path: &PathBuf) -> Result<String, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read plan file: {}", e))?;
    let detail: PlanDetail =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse plan file: {}", e))?;
    Ok(detail.markdown_content)
}
