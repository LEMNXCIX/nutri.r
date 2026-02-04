use crate::models::UIPreferences;
use crate::utils::error::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

pub trait PreferencesRepository {
    fn get(&self) -> AppResult<UIPreferences>;
    fn save(&self, preferences: &UIPreferences) -> AppResult<()>;
}

#[derive(Clone)]
pub struct FilePreferencesRepository {
    file_path: PathBuf,
}

impl FilePreferencesRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    fn default_preferences() -> UIPreferences {
        UIPreferences {
            theme: "light".to_string(),
            primary_color: "green".to_string(),
        }
    }
}

impl PreferencesRepository for FilePreferencesRepository {
    fn get(&self) -> AppResult<UIPreferences> {
        if !self.file_path.exists() {
            return Ok(Self::default_preferences());
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| AppError::Database(format!("Failed to read preferences: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| AppError::Serialization(format!("Failed to parse preferences: {}", e)))
    }

    fn save(&self, preferences: &UIPreferences) -> AppResult<()> {
        let json = serde_json::to_string_pretty(preferences).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize preferences: {}", e))
        })?;

        fs::write(&self.file_path, json)
            .map_err(|e| AppError::Database(format!("Failed to save preferences: {}", e)))
    }
}
