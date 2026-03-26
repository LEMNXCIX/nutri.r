use tracing::info;

use crate::models::{PlanDetail, PlanIndex, StructuredDay, StructuredPlan};
use crate::utils::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

/// Trait for plan data access
pub trait PlanRepository {
    fn get_all(&self) -> AppResult<Vec<PlanIndex>>;
    fn get_by_id(&self, id: &str) -> AppResult<PlanDetail>;
    fn get_index_by_id(&self, id: &str) -> AppResult<PlanIndex>;
    fn save(&self, detail: &PlanDetail) -> AppResult<String>;
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

    fn markdown_path(&self, id: &str) -> PathBuf {
        self.data_dir.join(format!("{}.md", id))
    }

    fn legacy_json_path(&self, id: &str) -> PathBuf {
        self.data_dir.join(format!("{}.json", id))
    }

    fn structured_path(&self, id: &str) -> PathBuf {
        self.data_dir.join(format!("{}.structured.json", id))
    }

    fn parse_structured_plan(&self, content: &str) -> Option<StructuredPlan> {
        let trimmed = content.trim();
        let maybe_json = if (trimmed.starts_with('{') && trimmed.ends_with('}'))
            || (trimmed.starts_with('[') && trimmed.ends_with(']'))
        {
            Some(trimmed.to_string())
        } else {
            let start = trimmed.find('{').or_else(|| trimmed.find('['))?;
            let end = trimmed.rfind('}').or_else(|| trimmed.rfind(']'))?;
            Some(trimmed[start..=end].to_string())
        }?;

        serde_json::from_str::<StructuredPlan>(&maybe_json)
            .map(StructuredPlan::normalized)
            .or_else(|_| {
                serde_json::from_str::<Vec<StructuredDay>>(&maybe_json).map(|days| {
                    StructuredPlan {
                        title: "Plan Nutricional".to_string(),
                        instructions: None,
                        days,
                    }
                    .normalized()
                })
            })
            .ok()
    }

    fn load_structured_plan(
        &self,
        id: &str,
        markdown_content: &str,
    ) -> AppResult<Option<StructuredPlan>> {
        let structured_path = self.structured_path(id);
        if structured_path.exists() {
            let structured_content = fs::read_to_string(&structured_path).map_err(|e| {
                AppError::Database(format!("Failed to read structured plan: {}", e))
            })?;
            let structured =
                serde_json::from_str::<StructuredPlan>(&structured_content).map_err(|e| {
                    AppError::Serialization(format!("Failed to parse structured plan: {}", e))
                })?;
            return Ok(Some(structured.normalized()));
        }

        Ok(self.parse_structured_plan(markdown_content))
    }

    fn write_plan_detail(&self, detail: &PlanDetail) -> AppResult<()> {
        fs::create_dir_all(&self.data_dir)
            .map_err(|e| AppError::Database(format!("Failed to create data directory: {}", e)))?;

        let markdown_content = if detail.markdown_content.trim().is_empty() {
            detail
                .structured_plan
                .as_ref()
                .map(StructuredPlan::to_markdown)
                .unwrap_or_default()
        } else {
            detail.markdown_content.clone()
        };

        fs::write(self.markdown_path(&detail.id), markdown_content)
            .map_err(|e| AppError::Database(format!("Failed to write plan: {}", e)))?;

        let structured_path = self.structured_path(&detail.id);
        if let Some(structured_plan) = detail.structured_plan.clone() {
            let json = serde_json::to_string_pretty(&structured_plan.normalized())?;
            fs::write(structured_path, json).map_err(|e| {
                AppError::Database(format!("Failed to write structured plan: {}", e))
            })?;
        } else if structured_path.exists() {
            fs::remove_file(structured_path).map_err(|e| {
                AppError::Database(format!("Failed to remove structured plan: {}", e))
            })?;
        }

        Ok(())
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
        let plan_path_md = self.markdown_path(id);
        if plan_path_md.exists() {
            let markdown_content = fs::read_to_string(&plan_path_md)
                .map_err(|e| AppError::Database(format!("Failed to read plan: {}", e)))?;
            let structured_plan = self.load_structured_plan(id, &markdown_content)?;
            return Ok(PlanDetail {
                id: id.to_string(),
                markdown_content,
                structured_plan,
            });
        }

        let plan_path_json = self.legacy_json_path(id);
        if plan_path_json.exists() {
            let content_json = fs::read_to_string(&plan_path_json)
                .map_err(|e| AppError::Database(format!("Failed to read legacy plan: {}", e)))?;

            #[derive(serde::Deserialize)]
            struct LegacyPlan {
                markdown_content: String,
                #[serde(default)]
                structured_plan: Option<StructuredPlan>,
            }

            let legacy: LegacyPlan = serde_json::from_str(&content_json).map_err(|e| {
                AppError::Serialization(format!("Failed to parse legacy plan: {}", e))
            })?;

            let structured_plan = legacy
                .structured_plan
                .map(StructuredPlan::normalized)
                .or_else(|| self.parse_structured_plan(&legacy.markdown_content));

            return Ok(PlanDetail {
                id: id.to_string(),
                markdown_content: legacy.markdown_content,
                structured_plan,
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

    fn save(&self, detail: &PlanDetail) -> AppResult<String> {
        let now = chrono::Local::now();
        let plan_id = now.format("%Y%m%d%H%M%S").to_string();
        let mut detail = detail.clone();
        detail.id = plan_id.clone();
        self.write_plan_detail(&detail)?;
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
        let created_at = chrono::Utc::now().to_rfc3339();
        let new_entry = PlanIndex {
            id: plan_id.to_string(),
            fecha: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            created_at: Some(created_at),
            display_name: None,
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
        self.write_plan_detail(&detail)
    }

    fn delete_plan(&self, id: &str) -> AppResult<()> {
        let plan_path_md = self.markdown_path(id);
        if plan_path_md.exists() {
            fs::remove_file(&plan_path_md)
                .map_err(|e| AppError::Database(format!("Failed to delete plan file: {}", e)))?;
        }

        let plan_path_json = self.legacy_json_path(id);
        if plan_path_json.exists() {
            fs::remove_file(&plan_path_json).map_err(|e| {
                AppError::Database(format!("Failed to delete legacy plan file: {}", e))
            })?;
        }

        let structured_path = self.structured_path(id);
        if structured_path.exists() {
            fs::remove_file(&structured_path).map_err(|e| {
                AppError::Database(format!("Failed to delete structured plan file: {}", e))
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
