use crate::models::achievement::{Achievement, AchievementType};
use crate::utils::error::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

pub trait AchievementRepository {
    fn get_all(&self) -> AppResult<Vec<Achievement>>;
    fn save_all(&self, achievements: &[Achievement]) -> AppResult<()>;
    fn unlock(&self, id: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct FileAchievementRepository {
    file_path: PathBuf,
}

impl FileAchievementRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    fn default_achievements() -> Vec<Achievement> {
        vec![
            Achievement {
                id: "first_plan".to_string(),
                title: "Primeros Pasos".to_string(),
                description: "Genera tu primer plan nutricional".to_string(),
                icon: "🌱".to_string(),
                unlocked_at: None,
                condition_type: AchievementType::PlanCreated,
                condition_value: 1,
            },
            Achievement {
                id: "picky_eater".to_string(),
                title: "Paladar Exigente".to_string(),
                description: "Excluye 5 ingredientes de tus dietas".to_string(),
                icon: "🚫".to_string(),
                unlocked_at: None,
                condition_type: AchievementType::IngredientExcluded,
                condition_value: 5,
            },
            Achievement {
                id: "shopper".to_string(),
                title: "Comprador Compulsivo".to_string(),
                description: "Genera 3 listas de compras".to_string(),
                icon: "🛒".to_string(),
                unlocked_at: None,
                condition_type: AchievementType::ShoppingListGenerated,
                condition_value: 3,
            },
            Achievement {
                id: "critic".to_string(),
                title: "Crítico Gastronómico".to_string(),
                description: "Califica un plan con estrellas".to_string(),
                icon: "⭐".to_string(),
                unlocked_at: None,
                condition_type: AchievementType::PlanRated,
                condition_value: 1,
            },
        ]
    }
}

impl AchievementRepository for FileAchievementRepository {
    fn get_all(&self) -> AppResult<Vec<Achievement>> {
        if !self.file_path.exists() {
            let defaults = Self::default_achievements();
            self.save_all(&defaults)?;
            return Ok(defaults);
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| AppError::Database(format!("Failed to read achievements: {}", e)))?;

        // If file is empty or invalid, return defaults
        if content.is_empty() {
            let defaults = Self::default_achievements();
            self.save_all(&defaults)?;
            return Ok(defaults);
        }

        serde_json::from_str(&content)
            .map_err(|e| AppError::Serialization(format!("Failed to parse achievements: {}", e)))
    }

    fn save_all(&self, achievements: &[Achievement]) -> AppResult<()> {
        let json = serde_json::to_string_pretty(achievements).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize achievements: {}", e))
        })?;

        fs::write(&self.file_path, json)
            .map_err(|e| AppError::Database(format!("Failed to save achievements: {}", e)))
    }

    fn unlock(&self, id: &str) -> AppResult<()> {
        let mut achievements = self.get_all()?;
        if let Some(achievement) = achievements.iter_mut().find(|a| a.id == id) {
            if achievement.unlocked_at.is_none() {
                achievement.unlocked_at = Some(chrono::Utc::now().to_rfc3339());
                self.save_all(&achievements)?;
            }
        }
        Ok(())
    }
}
