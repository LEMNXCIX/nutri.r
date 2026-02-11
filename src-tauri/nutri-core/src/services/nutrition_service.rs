use crate::models::{NutritionalInfo, PlanNutrition};
use crate::repositories::{ConfigRepository, PlanRepository};
use crate::utils::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct USDASearchResponse {
    foods: Vec<USDAFood>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct USDAFood {
    #[serde(rename = "fdcId")]
    fdc_id: i32,
    description: String,
    #[serde(rename = "foodNutrients")]
    food_nutrients: Vec<USDANutrient>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct USDANutrient {
    #[serde(rename = "nutrientId")]
    id: i32,
    #[serde(rename = "nutrientName")]
    name: String,
    value: f32,
    unit_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct NutritionCache {
    ingredients: HashMap<String, NutritionalInfo>,
}

pub struct NutritionService<P, C>
where
    P: PlanRepository,
    C: ConfigRepository,
{
    plan_repo: P,
    config_repo: C,
    cache_path: PathBuf,
}

impl<P, C> NutritionService<P, C>
where
    P: PlanRepository,
    C: ConfigRepository,
{
    pub fn new(plan_repo: P, config_repo: C, data_dir: PathBuf) -> Self {
        Self {
            plan_repo,
            config_repo,
            cache_path: data_dir.join("nutrition_cache.json"),
        }
    }

    fn load_cache(&self) -> NutritionCache {
        if !self.cache_path.exists() {
            return NutritionCache::default();
        }
        let content = fs::read_to_string(&self.cache_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    }

    fn save_cache(&self, cache: &NutritionCache) -> AppResult<()> {
        let json = serde_json::to_string_pretty(cache)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize cache: {}", e)))?;
        fs::write(&self.cache_path, json)
            .map_err(|e| AppError::Database(format!("Failed to write cache: {}", e)))?;
        Ok(())
    }

    pub async fn get_nutrition_for_ingredient(&self, name: &str) -> AppResult<NutritionalInfo> {
        let mut cache = self.load_cache();
        if let Some(info) = cache.ingredients.get(name) {
            return Ok(info.clone());
        }

        let config = self.config_repo.get()?;
        if config.usda_api_key.is_empty() {
            return Err(AppError::Configuration(
                "USDA API Key is missing. Please configure it in settings.".to_string(),
            ));
        }

        let client = reqwest::Client::new();
        let url = format!(
            "https://api.nal.usda.gov/fdc/v1/foods/search?api_key={}&query={}&pageSize=1",
            config.usda_api_key, name
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("USDA API error: {}", e)))?;

        let search_res: USDASearchResponse = response.json().await.map_err(|e| {
            AppError::Serialization(format!("Failed to parse USDA response: {}", e))
        })?;

        if let Some(food) = search_res.foods.first() {
            let mut info = NutritionalInfo::default();
            for n in &food.food_nutrients {
                match n.id {
                    1008 | 2047 => info.calories = n.value, // Energy (kcal)
                    1003 => info.protein = n.value,         // Protein
                    1005 => info.carbohydrates = n.value,   // Carbohydrates
                    1004 => info.fat = n.value,             // Total lipid (fat)
                    _ => {}
                }
            }

            cache.ingredients.insert(name.to_string(), info.clone());
            let _ = self.save_cache(&cache);
            Ok(info)
        } else {
            Err(AppError::NotFound(format!(
                "Ingredient '{}' not found in USDA database",
                name
            )))
        }
    }

    pub async fn get_plan_nutrition(&self, plan_id: &str) -> AppResult<PlanNutrition> {
        let plan_idx = self.plan_repo.get_index_by_id(plan_id)?;
        let mut nutrition = PlanNutrition {
            plan_id: plan_id.to_string(),
            ..Default::default()
        };

        // For simplicity, we analyze proteins mentioned in ingredients
        for protein in plan_idx.proteinas {
            if let Ok(info) = self.get_nutrition_for_ingredient(&protein).await {
                nutrition.total_calories += info.calories;
                nutrition.total_protein += info.protein;
                nutrition.total_carbs += info.carbohydrates;
                nutrition.total_fat += info.fat;
                nutrition
                    .breakdown_by_item
                    .insert(protein.to_string(), info);
            }
        }

        Ok(nutrition)
    }
}
