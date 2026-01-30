use crate::models::metadata::PlanMetadata;
use crate::utils::{AppError, AppResult};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub trait MetadataRepository {
    fn get(&self, plan_id: &str) -> AppResult<Option<PlanMetadata>>;
    fn save(&self, metadata: PlanMetadata) -> AppResult<()>;
    fn get_all(&self) -> AppResult<Vec<PlanMetadata>>;
}

pub struct FileMetadataRepository {
    data_dir: PathBuf,
}

impl FileMetadataRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    fn file_path(&self) -> PathBuf {
        self.data_dir.join("metadata.json")
    }

    fn read_store(&self) -> AppResult<HashMap<String, PlanMetadata>> {
        let path = self.file_path();
        if !path.exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| AppError::Database(format!("Failed to read metadata store: {}", e)))?;

        let store: HashMap<String, PlanMetadata> = serde_json::from_str(&content).map_err(|e| {
            AppError::Serialization(format!("Failed to parse metadata store: {}", e))
        })?;

        Ok(store)
    }

    fn write_store(&self, store: &HashMap<String, PlanMetadata>) -> AppResult<()> {
        let path = self.file_path();
        let content = serde_json::to_string_pretty(store).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize metadata store: {}", e))
        })?;

        fs::write(&path, content)
            .map_err(|e| AppError::Database(format!("Failed to write metadata store: {}", e)))?;

        Ok(())
    }
}

impl MetadataRepository for FileMetadataRepository {
    fn get(&self, plan_id: &str) -> AppResult<Option<PlanMetadata>> {
        let store = self.read_store()?;
        Ok(store.get(plan_id).cloned())
    }

    fn save(&self, metadata: PlanMetadata) -> AppResult<()> {
        let mut store = self.read_store()?;
        store.insert(metadata.plan_id.clone(), metadata);
        self.write_store(&store)
    }

    fn get_all(&self) -> AppResult<Vec<PlanMetadata>> {
        let store = self.read_store()?;
        Ok(store.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_metadata_operations() {
        let temp_dir = env::temp_dir().join("nutri_r_metadata_test");
        let _ = fs::create_dir_all(&temp_dir); // Ensure exists
        let repo = FileMetadataRepository::new(temp_dir.clone());

        let meta = PlanMetadata {
            plan_id: "plan1".to_string(),
            is_favorite: true,
            rating: Some(5),
            notes: "Great plan".to_string(),
            tags: vec![],
        };

        repo.save(meta.clone()).unwrap();

        let retrieved = repo.get("plan1").unwrap().unwrap();
        assert_eq!(retrieved.plan_id, "plan1");
        assert!(retrieved.is_favorite);

        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 1);

        let _ = fs::remove_dir_all(temp_dir);
    }
}
