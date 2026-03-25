use tracing::info;

use crate::models::{
    derive_plan_display_name, metadata::PlanMetadata, PlanDetail, PlanIndex, RecipeSuggestion,
    StructuredPlan, StructuredRecipe, VariationType,
};
use crate::repositories::{
    ConfigRepository, IngredientRepository, MetadataRepository, PantryRepository, PlanRepository,
};
use crate::services::ai_service::OllamaService;
use crate::utils::{AppError, AppResult};

pub struct PlanService<P, C, I, PA, M>
where
    P: PlanRepository,
    C: ConfigRepository,
    I: IngredientRepository,
    PA: PantryRepository,
    M: MetadataRepository,
{
    plan_repo: P,
    config_repo: C,
    ingredient_repo: I,
    pantry_repo: PA,
    metadata_repo: M,
    ai_service: OllamaService,
}

impl<P, C, I, PA, M> PlanService<P, C, I, PA, M>
where
    P: PlanRepository,
    C: ConfigRepository,
    I: IngredientRepository,
    PA: PantryRepository,
    M: MetadataRepository,
{
    pub fn new(
        plan_repo: P,
        config_repo: C,
        ingredient_repo: I,
        pantry_repo: PA,
        metadata_repo: M,
    ) -> Self {
        Self {
            plan_repo,
            config_repo,
            ingredient_repo,
            pantry_repo,
            metadata_repo,
            ai_service: OllamaService::new(),
        }
    }

    fn sync_plan_index(
        &self,
        plan_id: &str,
        structured_plan: &StructuredPlan,
    ) -> AppResult<()> {
        let mut index = self.plan_repo.get_all()?;
        if let Some(entry) = index.iter_mut().find(|plan| plan.id == plan_id) {
            entry.weekly_structure = Some(structured_plan.to_weekly_structure());
        }
        self.plan_repo.overwrite_index(index)
    }

    fn resolve_display_name(
        &self,
        plan: &PlanIndex,
        metadata: Option<&PlanMetadata>,
        structured_plan: Option<&StructuredPlan>,
    ) -> String {
        if let Some(custom_name) = metadata
            .and_then(|meta| meta.display_name.as_ref())
            .map(|name| name.trim())
            .filter(|name| !name.is_empty())
        {
            return custom_name.to_string();
        }

        let created_at = plan
            .created_at
            .as_deref()
            .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
            .map(|value| value.with_timezone(&chrono::Utc));

        derive_plan_display_name(
            structured_plan,
            created_at.as_ref(),
            &plan.proteinas,
            &plan.id,
        )
    }

    fn persist_initial_display_name(
        &self,
        plan_id: &str,
        proteins: &[String],
        structured_plan: Option<&StructuredPlan>,
    ) -> AppResult<()> {
        let index = self.plan_repo.get_index_by_id(plan_id)?;
        let created_at = index
            .created_at
            .as_deref()
            .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
            .map(|value| value.with_timezone(&chrono::Utc));
        let display_name =
            derive_plan_display_name(structured_plan, created_at.as_ref(), proteins, plan_id);

        let mut metadata = self
            .metadata_repo
            .get(plan_id)?
            .unwrap_or_else(|| PlanMetadata {
                plan_id: plan_id.to_string(),
                ..Default::default()
            });
        metadata.display_name = Some(display_name);
        self.metadata_repo.save(metadata)?;

        Ok(())
    }

    /// Generate a new meal plan
    pub async fn generate_plan(&self) -> AppResult<String> {
        // Get configuration
        let config = self.config_repo.get()?;

        // Validate configuration
        if config.ollama_url.is_empty() {
            return Err(AppError::Configuration(
                "Ollama URL is not configured".to_string(),
            ));
        }
        if config.ollama_model.is_empty() {
            return Err(AppError::Configuration(
                "Ollama model is not configured".to_string(),
            ));
        }

        // Get excluded ingredients
        let excluded = self.ingredient_repo.get_excluded()?;
        let exclusion_list = excluded.ingredients.join(", ");

        // Get pantry items
        let pantry_items = self.pantry_repo.get_all()?;
        let pantry_list = pantry_items
            .iter()
            .filter(|i| i.quantity > 0.0)
            .map(|i| format!("{} ({} {})", i.name, i.quantity, i.unit))
            .collect::<Vec<String>>()
            .join(", ");

        log::info!(
            "Generating plan with exclusions: {} and pantry: {}",
            exclusion_list,
            pantry_list
        );

        // Generate plan using AI
        let (plan_content, proteins, weekly_structure, structured_plan) = self
            .ai_service
            .generate_plan(
                &config.ollama_url,
                &config.ollama_model,
                config.prompt_maestro,
                exclusion_list,
                pantry_list,
            )
            .await?;

        let structured_plan = structured_plan.map(StructuredPlan::normalized);
        let detail = PlanDetail {
            id: String::new(),
            markdown_content: plan_content,
            structured_plan: structured_plan.clone(),
        };
        let plan_id = self.plan_repo.save(&detail)?;

        // Update index
        self.plan_repo
            .update_index(
                &plan_id,
                proteins.clone(),
                weekly_structure
                    .or_else(|| structured_plan.clone().map(|plan| plan.to_weekly_structure())),
            )?;
        self.persist_initial_display_name(
            &plan_id,
            &proteins,
            detail.structured_plan.as_ref(),
        )?;

        log::info!("Plan generated successfully: {}", plan_id);

        Ok(plan_id)
    }

    /// Get all plans
    pub fn list_plans(&self) -> AppResult<Vec<PlanIndex>> {
        info!("listando planes");
        let mut plans = self.plan_repo.get_all()?;

        for plan in &mut plans {
            let metadata = self.metadata_repo.get(&plan.id)?;
            let structured_plan = if metadata
                .as_ref()
                .and_then(|meta| meta.display_name.as_ref())
                .is_some()
            {
                None
            } else {
                self.plan_repo
                    .get_by_id(&plan.id)
                    .ok()
                    .and_then(|detail| detail.structured_plan)
            };

            if let Some(meta) = metadata.as_ref() {
                plan.is_favorite = meta.is_favorite;
                plan.rating = meta.rating;
            }

            plan.display_name = Some(self.resolve_display_name(
                plan,
                metadata.as_ref(),
                structured_plan.as_ref(),
            ));
        }

        Ok(plans)
    }

    /// Get plan content by ID
    pub fn get_plan_content(&self, plan_id: &str) -> AppResult<String> {
        let plan = self.plan_repo.get_by_id(plan_id)?;
        Ok(plan.markdown_content)
    }

    pub fn get_plan_detail(&self, plan_id: &str) -> AppResult<PlanDetail> {
        self.plan_repo.get_by_id(plan_id)
    }

    /// Delete a plan by ID
    pub fn delete_plan(&self, plan_id: &str) -> AppResult<()> {
        log::info!("Deleting plan {}", plan_id);
        self.plan_repo.delete_plan(plan_id)?;
        log::info!("Plan {} deleted successfully", plan_id);
        Ok(())
    }

    /// Generate a variation of an existing plan
    pub async fn generate_variation(
        &self,
        plan_id: &str,
        variation: VariationType,
    ) -> AppResult<String> {
        // Get original content
        let original_content = self.get_plan_content(plan_id)?;

        // Get configuration
        let config = self.config_repo.get()?;

        // Validate configuration
        if config.ollama_url.is_empty() {
            return Err(AppError::Configuration(
                "Ollama URL is not configured".to_string(),
            ));
        }
        if config.ollama_model.is_empty() {
            return Err(AppError::Configuration(
                "Ollama model is not configured".to_string(),
            ));
        }

        let prompt = format!(
            "Toma el siguiente plan nutricional y genera una VARIACIÓN DE TIPO: {}.\n\n\
            REGLAS:\n\
            1. Manten exactamente la misma estructura de días y comidas (Desayuno, Almuerzo, Cena).\n\
            2. Adapta TODOS los ingredientes para que cumplan estrictamente con el tipo: {}.\n\
            3. Si el original tiene carne y la variación es Vegana, sustituye por proteínas vegetales de valor equivalente.\n\
            4. Si la variación es Keto, prioriza grasas y proteínas, eliminando carbohidratos simples (pan, arroz, azúcar).\n\
            5. Si la variación es Gluten-Free, sustituye cereales con gluten por alternativas seguras.\n\
            6. Responde ÚNICAMENTE con el nuevo plan adaptado en formato Markdown, similar al estilo del original.\n\n\
            PLAN ORIGINAL A ADAPTAR:\n\n{}",
            variation, variation, original_content
        );

        log::info!("Generating {} variation for plan {}", variation, plan_id);

        // Generate variation using AI
        let (plan_content, proteins, weekly_structure, structured_plan) = self
            .ai_service
            .generate_plan(
                &config.ollama_url,
                &config.ollama_model,
                prompt,
                String::new(), // In transformations, the prompt itself guides the adaptation
                String::new(), // No pantry injection for variations yet unless needed
            )
            .await?;

        let structured_plan = structured_plan.map(StructuredPlan::normalized);
        let detail = PlanDetail {
            id: String::new(),
            markdown_content: plan_content,
            structured_plan: structured_plan.clone(),
        };
        let new_plan_id = self.plan_repo.save(&detail)?;

        // Update index with new proteins
        self.plan_repo
            .update_index(
                &new_plan_id,
                proteins.clone(),
                weekly_structure
                    .or_else(|| structured_plan.clone().map(|plan| plan.to_weekly_structure())),
            )?;
        self.persist_initial_display_name(
            &new_plan_id,
            &proteins,
            detail.structured_plan.as_ref(),
        )?;

        log::info!(
            "Variation generated successfully: {} -> {}",
            plan_id,
            new_plan_id
        );

        Ok(new_plan_id)
    }

    pub async fn suggest_recipe_edit(
        &self,
        plan_id: &str,
        recipe_id: &str,
        prompt: &str,
    ) -> AppResult<RecipeSuggestion> {
        let mut detail = self.plan_repo.get_by_id(plan_id)?;
        let structured_plan = detail
            .structured_plan
            .take()
            .map(StructuredPlan::normalized)
            .ok_or_else(|| {
                AppError::Validation(
                    "El plan no tiene estructura editable; regenera o convierte este plan."
                        .to_string(),
                )
            })?;

        let mut located: Option<(crate::models::StructuredDay, crate::models::StructuredRecipe)> = None;
        for day in &structured_plan.days {
            if let Some(recipe) = day.recipes.iter().find(|recipe| recipe.recipe_id == recipe_id) {
                located = Some((day.clone(), recipe.clone()));
                break;
            }
        }

        let (day, recipe) = located.ok_or_else(|| {
            AppError::NotFound(format!("Recipe {} not found in plan {}", recipe_id, plan_id))
        })?;

        let config = self.config_repo.get()?;
        if config.ollama_url.is_empty() || config.ollama_model.is_empty() {
            return Err(AppError::Configuration(
                "Ollama no está configurado para sugerir ediciones".to_string(),
            ));
        }

        self.ai_service
            .suggest_recipe_edit(
                &config.ollama_url,
                &config.ollama_model,
                plan_id,
                &structured_plan,
                &day,
                &recipe,
                prompt,
            )
            .await
    }

    pub fn apply_recipe_edit(
        &self,
        plan_id: &str,
        recipe_id: &str,
        mut suggested_recipe: StructuredRecipe,
    ) -> AppResult<PlanDetail> {
        let mut detail = self.plan_repo.get_by_id(plan_id)?;
        let mut structured_plan = detail
            .structured_plan
            .take()
            .map(StructuredPlan::normalized)
            .ok_or_else(|| {
                AppError::Validation(
                    "El plan no tiene estructura editable; no se puede aplicar un patch atómico."
                        .to_string(),
                )
            })?;

        let mut original_meal_type = None;
        let mut replaced = false;
        for day in &mut structured_plan.days {
            if let Some(recipe) = day
                .recipes
                .iter_mut()
                .find(|recipe| recipe.recipe_id == recipe_id)
            {
                original_meal_type = Some(recipe.meal_type.clone());
                suggested_recipe.recipe_id = recipe.recipe_id.clone();
                recipe.name = suggested_recipe.name.clone();
                recipe.ingredients = suggested_recipe.ingredients.clone();
                recipe.instructions = suggested_recipe.instructions.clone();
                recipe.notes = suggested_recipe.notes.clone();
                replaced = true;
                break;
            }
        }

        if !replaced {
            return Err(AppError::NotFound(format!(
                "Recipe {} not found in plan {}",
                recipe_id, plan_id
            )));
        }

        if let Some(meal_type) = original_meal_type {
            for day in &mut structured_plan.days {
                if let Some(recipe) = day
                    .recipes
                    .iter_mut()
                    .find(|recipe| recipe.recipe_id == recipe_id)
                {
                    recipe.meal_type = meal_type;
                    break;
                }
            }
        }

        structured_plan = structured_plan.normalized();
        detail.structured_plan = Some(structured_plan.clone());
        detail.markdown_content = structured_plan.to_markdown();
        self.plan_repo.save_detail(detail.clone())?;
        self.sync_plan_index(plan_id, &structured_plan)?;

        Ok(detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        FileConfigRepository, FileIngredientRepository, FilePantryRepository, FilePlanRepository,
    };
    use std::env;

    #[test]
    fn test_plan_service_creation() {
        let temp_dir = env::temp_dir().join("nutri_r_test_service");
        let plan_repo = FilePlanRepository::new(temp_dir.clone());
        let config_repo = FileConfigRepository::new(temp_dir.join("config.json"));
        let ingredient_repo = FileIngredientRepository::new(temp_dir.join("excluded.json"));
        let pantry_repo = FilePantryRepository::new(temp_dir.clone());
        let metadata_repo = crate::repositories::FileMetadataRepository::new(temp_dir.clone());

        let _service = PlanService::new(
            plan_repo,
            config_repo,
            ingredient_repo,
            pantry_repo,
            metadata_repo,
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
