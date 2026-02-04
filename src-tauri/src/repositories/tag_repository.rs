use crate::models::Tag;
use crate::utils::AppResult;
use std::fs;
use std::path::{Path, PathBuf};

pub trait TagRepository: Send + Sync {
    fn get_all(&self) -> AppResult<Vec<Tag>>;
    fn save_all(&self, tags: Vec<Tag>) -> AppResult<()>;
}

#[derive(Clone)]
pub struct FileTagRepository {
    path: PathBuf,
}

impl FileTagRepository {
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        Self {
            path: data_dir.as_ref().join("tags.json"),
        }
    }

    fn ensure_file_exists(&self) -> AppResult<()> {
        if !self.path.exists() {
            if let Some(parent) = self.path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&self.path, "[]")?;
        }
        Ok(())
    }
}

impl TagRepository for FileTagRepository {
    fn get_all(&self) -> AppResult<Vec<Tag>> {
        self.ensure_file_exists()?;
        let content = fs::read_to_string(&self.path)?;
        let tags = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
        Ok(tags)
    }

    fn save_all(&self, tags: Vec<Tag>) -> AppResult<()> {
        let content = serde_json::to_string_pretty(&tags)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}
