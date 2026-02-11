use crate::models::PantryItem;
use crate::utils::error::AppResult;
use std::fs;
use std::path::PathBuf;

pub trait PantryRepository: Send + Sync {
    fn get_all(&self) -> AppResult<Vec<PantryItem>>;
    fn save_all(&self, items: Vec<PantryItem>) -> AppResult<()>;
}

#[derive(Clone)]
pub struct FilePantryRepository {
    path: PathBuf,
}

impl FilePantryRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            path: data_dir.join("pantry.json"),
        }
    }

    fn ensure_file(&self) -> AppResult<()> {
        if !self.path.exists() {
            fs::write(&self.path, "[]")?;
        }
        Ok(())
    }
}

impl PantryRepository for FilePantryRepository {
    fn get_all(&self) -> AppResult<Vec<PantryItem>> {
        self.ensure_file()?;
        let content = fs::read_to_string(&self.path)?;
        let items = serde_json::from_str(&content)?;
        Ok(items)
    }

    fn save_all(&self, items: Vec<PantryItem>) -> AppResult<()> {
        let content = serde_json::to_string_pretty(&items)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}
