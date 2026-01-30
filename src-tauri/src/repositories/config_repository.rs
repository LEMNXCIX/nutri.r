use crate::models::AppConfig;
use crate::utils::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

/// Trait for configuration data access
pub trait ConfigRepository {
    fn get(&self) -> AppResult<AppConfig>;
    fn save(&self, config: &AppConfig) -> AppResult<()>;
}

/// File-based implementation of ConfigRepository
#[derive(Clone)]
pub struct FileConfigRepository {
    config_path: PathBuf,
}

impl FileConfigRepository {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }
}

impl ConfigRepository for FileConfigRepository {
    fn get(&self) -> AppResult<AppConfig> {
        if !self.config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| AppError::Configuration(format!("Failed to read config: {}", e)))?;

        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| AppError::Serialization(format!("Failed to parse config: {}", e)))?;

        Ok(config)
    }

    fn save(&self, config: &AppConfig) -> AppResult<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                AppError::Configuration(format!("Failed to create config directory: {}", e))
            })?;
        }

        let json = serde_json::to_string_pretty(config)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize config: {}", e)))?;

        fs::write(&self.config_path, json)
            .map_err(|e| AppError::Configuration(format!("Failed to write config: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_save_and_get_config() {
        let temp_file = env::temp_dir().join("test_config.json");
        let repo = FileConfigRepository::new(temp_file.clone());

        let mut config = AppConfig::default();
        config.smtp_host = "smtp.test.com".to_string();

        repo.save(&config).unwrap();
        let retrieved = repo.get().unwrap();

        assert_eq!(retrieved.smtp_host, "smtp.test.com");

        // Cleanup
        let _ = fs::remove_file(temp_file);
    }
}
