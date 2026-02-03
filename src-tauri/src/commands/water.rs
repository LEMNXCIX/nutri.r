use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::State;

use crate::models::WaterRecord;
use crate::state::AppState;

type WaterData = HashMap<String, WaterRecord>;

fn get_water_file_path(state: &AppState) -> PathBuf {
    state.data_dir.join("water.json")
}

fn load_water_data(state: &AppState) -> WaterData {
    let path = get_water_file_path(state);
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_water_data(state: &AppState, data: &WaterData) -> Result<(), String> {
    let path = get_water_file_path(state);
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_water_intake(date: String, state: State<AppState>) -> WaterRecord {
    let data = load_water_data(&state);
    data.get(&date).cloned().unwrap_or_else(|| WaterRecord {
        current: 0.0,
        target: 2.5,
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
pub fn update_water_intake(
    date: String,
    current: f32,
    target: f32,
    state: State<AppState>,
) -> Result<(), String> {
    let mut data = load_water_data(&state);
    data.insert(
        date,
        WaterRecord {
            current,
            target,
            last_updated: chrono::Utc::now().to_rfc3339(),
        },
    );
    save_water_data(&state, &data)
}

#[tauri::command]
pub fn get_water_history(
    start_date: String,
    end_date: String,
    state: State<AppState>,
) -> HashMap<String, WaterRecord> {
    let data = load_water_data(&state);
    data.into_iter()
        .filter(|(date, _)| date >= &start_date && date <= &end_date)
        .collect()
}
