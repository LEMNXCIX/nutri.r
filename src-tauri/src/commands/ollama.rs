use crate::models::{AppConfig, OllamaModelInfo};
use crate::repositories::ConfigRepository;
use crate::services::OllamaService;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_ollama_models(
    state: State<'_, AppState>,
) -> Result<Vec<OllamaModelInfo>, String> {
    // We need the config to get the URL
    let config = state.config_repo.get().unwrap_or(AppConfig::default());
    let service = OllamaService::new();

    service
        .list_models(&config.ollama_url)
        .await
        .map_err(|e| e.to_string())
}
