use tracing::info;

use crate::models::{PlanIndex, VariationType};
use crate::repositories::{
    ConfigRepository, IngredientRepository, PantryRepository, PlanRepository,
};
use crate::services::ai_service::OllamaService;
use crate::utils::{AppError, AppResult};

/// Service for plan management and generation
pub struct PlanService<P, C, I, PA>
where
    P: PlanRepository,
    C: ConfigRepository,
    I: IngredientRepository,
    PA: PantryRepository,
{
    plan_repo: P,
    config_repo: C,
    ingredient_repo: I,
    pantry_repo: PA,
    ai_service: OllamaService,
}

impl<P, C, I, PA> PlanService<P, C, I, PA>
where
    P: PlanRepository,
    C: ConfigRepository,
    I: IngredientRepository,
    PA: PantryRepository,
{
    pub fn new(plan_repo: P, config_repo: C, ingredient_repo: I, pantry_repo: PA) -> Self {
        Self {
            plan_repo,
            config_repo,
            ingredient_repo,
            pantry_repo,
            ai_service: OllamaService::new(),
        }
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
        let (plan_content, proteins, weekly_structure) = self
            .ai_service
            .generate_plan(
                &config.ollama_url,
                &config.ollama_model,
                config.prompt_maestro,
                exclusion_list,
                pantry_list,
            )
            .await?;

        // Save plan
        let plan_id = self.plan_repo.save(&plan_content)?;

        // Update index
        self.plan_repo
            .update_index(&plan_id, proteins, weekly_structure)?;

        log::info!("Plan generated successfully: {}", plan_id);

        Ok(plan_id)
    }

    /// Get all plans
    pub fn list_plans(&self) -> AppResult<Vec<PlanIndex>> {
        info!("listando planes");
        self.plan_repo.get_all()
    }

    /// Get plan content by ID
    pub fn get_plan_content(&self, plan_id: &str) -> AppResult<String> {
        let plan = self.plan_repo.get_by_id(plan_id)?;
        Ok(plan.markdown_content)
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
        let (plan_content, proteins, weekly_structure) = self
            .ai_service
            .generate_plan(
                &config.ollama_url,
                &config.ollama_model,
                prompt,
                String::new(), // In transformations, the prompt itself guides the adaptation
                String::new(), // No pantry injection for variations yet unless needed
            )
            .await?;

        // Save new plan
        let new_plan_id = self.plan_repo.save(&plan_content)?;

        // Update index with new proteins
        self.plan_repo
            .update_index(&new_plan_id, proteins, weekly_structure)?;

        log::info!(
            "Variation generated successfully: {} -> {}",
            plan_id,
            new_plan_id
        );

        Ok(new_plan_id)
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

        let _service = PlanService::new(plan_repo, config_repo, ingredient_repo, pantry_repo);

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
