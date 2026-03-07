use tracing::info;

use crate::models::{PlanDetail, PlanIndex};
use crate::utils::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

/// Trait for plan data access
pub trait PlanRepository {
    fn get_all(&self) -> AppResult<Vec<PlanIndex>>;
    fn get_by_id(&self, id: &str) -> AppResult<PlanDetail>;
    fn get_index_by_id(&self, id: &str) -> AppResult<PlanIndex>;
    fn save(&self, content: &str) -> AppResult<String>;
    fn update_index(
        &self,
        plan_id: &str,
        proteinas: Vec<String>,
        weekly_structure: Option<Vec<crate::models::WeeklyMealInfo>>,
    ) -> AppResult<()>;
    fn overwrite_index(&self, index: Vec<PlanIndex>) -> AppResult<()>;
    fn save_detail(&self, detail: PlanDetail) -> AppResult<()>;
    fn delete_plan(&self, id: &str) -> AppResult<()>;
}

/// File-based implementation of PlanRepository
#[derive(Clone)]
pub struct FilePlanRepository {
    data_dir: PathBuf,
}

impl FilePlanRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    fn index_path(&self) -> PathBuf {
        self.data_dir.join("index.json")
    }
}

impl PlanRepository for FilePlanRepository {
    fn get_all(&self) -> AppResult<Vec<PlanIndex>> {
        let index_path = self.index_path();
        info!("index_path {}", index_path.to_string_lossy());
        if !index_path.exists() {
            info!("no existe");
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&index_path)
            .map_err(|e| AppError::Database(format!("Failed to read index: {}", e)))?;

        let index: Vec<PlanIndex> = serde_json::from_str(&content)
            .map_err(|e| AppError::Serialization(format!("Failed to parse index: {}", e)))?;
        info!("planes {}", content);
        Ok(index)
    }

    fn get_by_id(&self, id: &str) -> AppResult<PlanDetail> {
        // Try new format (.md)
        let plan_path_md = self.data_dir.join(format!("{}.md", id));
        if plan_path_md.exists() {
            let markdown_content = fs::read_to_string(&plan_path_md)
                .map_err(|e| AppError::Database(format!("Failed to read plan: {}", e)))?;
            return Ok(PlanDetail {
                id: id.to_string(),
                markdown_content,
            });
        }

        // Try legacy format (.json)
        let plan_path_json = self.data_dir.join(format!("{}.json", id));
        if plan_path_json.exists() {
            let content_json = fs::read_to_string(&plan_path_json)
                .map_err(|e| AppError::Database(format!("Failed to read legacy plan: {}", e)))?;

            #[derive(serde::Deserialize)]
            struct LegacyPlan {
                markdown_content: String,
            }

            let legacy: LegacyPlan = serde_json::from_str(&content_json).map_err(|e| {
                AppError::Serialization(format!("Failed to parse legacy plan: {}", e))
            })?;

            return Ok(PlanDetail {
                id: id.to_string(),
                markdown_content: legacy.markdown_content,
            });
        }

        Err(AppError::NotFound(format!("Plan {} not found", id)))
    }

    fn get_index_by_id(&self, id: &str) -> AppResult<PlanIndex> {
        let index = self.get_all()?;
        index
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| AppError::NotFound(format!("Plan index entry {} not found", id)))
    }

    fn save(&self, content: &str) -> AppResult<String> {
        // Ensure data directory exists
        fs::create_dir_all(&self.data_dir)
            .map_err(|e| AppError::Database(format!("Failed to create data directory: {}", e)))?;

        // Generate unique ID using timestamp
        let now = chrono::Local::now();
        let plan_id = now.format("%Y%m%d%H%M%S").to_string();

        let plan_path = self.data_dir.join(format!("{}.md", plan_id));

        fs::write(&plan_path, content)
            .map_err(|e| AppError::Database(format!("Failed to write plan: {}", e)))?;

        Ok(plan_id)
    }

    fn update_index(
        &self,
        plan_id: &str,
        proteins: Vec<String>,
        weekly_structure: Option<Vec<crate::models::WeeklyMealInfo>>,
    ) -> AppResult<()> {
        let index_path = self.index_path();

        let mut index = if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            serde_json::from_str(&content)?
        } else {
            Vec::new()
        };

        let now = chrono::Local::now();
        let new_entry = PlanIndex {
            id: plan_id.to_string(),
            fecha: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            proteinas: proteins,
            enviado: false,
            is_favorite: false,
            rating: None,
            weekly_structure,
        };

        index.push(new_entry);

        let json = serde_json::to_string_pretty(&index)?;
        fs::write(&index_path, json)?;

        Ok(())
    }

    fn overwrite_index(&self, index: Vec<PlanIndex>) -> AppResult<()> {
        let index_path = self.index_path();
        let json = serde_json::to_string_pretty(&index)?;
        fs::write(&index_path, json)?;
        Ok(())
    }

    fn save_detail(&self, detail: PlanDetail) -> AppResult<()> {
        let plan_path = self.data_dir.join(format!("{}.md", detail.id));
        fs::write(&plan_path, detail.markdown_content)?;
        Ok(())
    }

    fn delete_plan(&self, id: &str) -> AppResult<()> {
        // Delete plan file
        let plan_path_md = self.data_dir.join(format!("{}.md", id));
        if plan_path_md.exists() {
            fs::remove_file(&plan_path_md)
                .map_err(|e| AppError::Database(format!("Failed to delete plan file: {}", e)))?;
        }

        let plan_path_json = self.data_dir.join(format!("{}.json", id));
        if plan_path_json.exists() {
            fs::remove_file(&plan_path_json).map_err(|e| {
                AppError::Database(format!("Failed to delete legacy plan file: {}", e))
            })?;
        }

        // Remove from index
        let index_path = self.index_path();
        if index_path.exists() {
            let mut index = self.get_all()?;
            index.retain(|p| p.id != id);

            let json = serde_json::to_string_pretty(&index)?;
            fs::write(&index_path, json).map_err(|e| {
                AppError::Database(format!("Failed to update index after deletion: {}", e))
            })?;
        }

        Ok(())
    }
}
