use crate::models::ExcludedIngredients;
use crate::utils::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

/// Trait for ingredient data access
pub trait IngredientRepository {
    fn get_excluded(&self) -> AppResult<ExcludedIngredients>;
    fn save_excluded(&self, excluded: &ExcludedIngredients) -> AppResult<()>;
}

/// File-based implementation of IngredientRepository
pub struct FileIngredientRepository {
    excluded_path: PathBuf,
}

impl FileIngredientRepository {
    pub fn new(excluded_path: PathBuf) -> Self {
        Self { excluded_path }
    }
}

impl IngredientRepository for FileIngredientRepository {
    fn get_excluded(&self) -> AppResult<ExcludedIngredients> {
        if !self.excluded_path.exists() {
            return Ok(ExcludedIngredients::default());
        }

        let content = fs::read_to_string(&self.excluded_path).map_err(|e| {
            AppError::Database(format!("Failed to read excluded ingredients: {}", e))
        })?;

        let excluded: ExcludedIngredients = serde_json::from_str(&content).map_err(|e| {
            AppError::Serialization(format!("Failed to parse excluded ingredients: {}", e))
        })?;

        Ok(excluded)
    }

    fn save_excluded(&self, excluded: &ExcludedIngredients) -> AppResult<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.excluded_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| AppError::Database(format!("Failed to create directory: {}", e)))?;
        }

        let json = serde_json::to_string_pretty(excluded).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize excluded ingredients: {}", e))
        })?;

        fs::write(&self.excluded_path, json).map_err(|e| {
            AppError::Database(format!("Failed to write excluded ingredients: {}", e))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_save_and_get_excluded() {
        let temp_file = env::temp_dir().join("test_excluded.json");
        let repo = FileIngredientRepository::new(temp_file.clone());

        let mut excluded = ExcludedIngredients::default();
        excluded.ingredients.push("Atún".to_string());
        excluded.ingredients.push("Salmón".to_string());

        repo.save_excluded(&excluded).unwrap();
        let retrieved = repo.get_excluded().unwrap();

        assert_eq!(retrieved.ingredients.len(), 2);
        assert!(retrieved.ingredients.contains(&"Atún".to_string()));

        // Cleanup
        let _ = fs::remove_file(temp_file);
    }
}
