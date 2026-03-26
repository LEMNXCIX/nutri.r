use crate::models::{ChatMessage, NutritionalInfo, PlanNutrition};
use crate::repositories::{ConfigRepository, PlanRepository, ShoppingListRepository};
use crate::services::ai_service::OllamaService;
use crate::utils::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

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
    #[serde(rename = "unitName")]
    unit_name: Option<String>, // Cambiado a Option para evitar errores si falta
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct NutritionCache {
    ingredients: HashMap<String, NutritionalInfo>,
    #[serde(default)]
    plans: HashMap<String, PlanNutrition>,
}

pub struct NutritionService<P, C, S>
where
    P: PlanRepository,
    C: ConfigRepository,
    S: ShoppingListRepository,
{
    plan_repo: P,
    config_repo: C,
    shopping_repo: S,
    ai_service: OllamaService,
    cache_path: PathBuf,
}

impl<P, C, S> NutritionService<P, C, S>
where
    P: PlanRepository,
    C: ConfigRepository,
    S: ShoppingListRepository,
{
    pub fn new(plan_repo: P, config_repo: C, shopping_repo: S, data_dir: PathBuf) -> Self {
        Self {
            plan_repo,
            config_repo,
            shopping_repo,
            ai_service: OllamaService::new(),
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
        self.get_nutrition_internal(&mut cache, name).await
    }

    async fn get_nutrition_internal(
        &self,
        cache: &mut NutritionCache,
        name: &str,
    ) -> AppResult<NutritionalInfo> {
        if let Some(info) = cache.ingredients.get(name) {
            info!("Found nutrition for '{}' in local cache", name);
            return Ok(info.clone());
        }

        let config = self.config_repo.get()?;
        if config.usda_api_key.is_empty() {
            error!("USDA API Key is missing in configuration");
            return Err(AppError::Configuration(
                "USDA API Key is missing. Please configure it in settings.".to_string(),
            ));
        }

        // STEP 1: Clean name
        let cleaned_name = if let Some(idx) = name.find('(') {
            name[..idx].trim().to_string()
        } else {
            name.to_string()
        };

        // STEP 2: Translate
        info!("Translating '{}' to English via Ollama", cleaned_name);
        let translation_prompt = format!(
            "Translate this food ingredient to a simple English term (one or two words only, no explanation): {}",
            cleaned_name
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a precise translator. Only output the English term.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: translation_prompt,
            },
        ];

        let english_name = self
            .ai_service
            .chat(&config.ollama_url, &config.ollama_model, messages)
            .await
            .unwrap_or_else(|_| cleaned_name.clone());

        let search_query = english_name.trim().trim_matches('"').to_string();

        // STEP 3: Fetch from USDA
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.nal.usda.gov/fdc/v1/foods/search?api_key={}&query={}&pageSize=1",
            config.usda_api_key, search_query
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("USDA API error: {}", e)))?;

        let search_res: USDASearchResponse = response.json().await.map_err(|e| {
            error!(
                "Failed to parse USDA response for '{}': {}",
                search_query, e
            );
            AppError::Serialization(format!("Failed to parse USDA response: {}", e))
        })?;

        if let Some(food) = search_res.foods.first() {
            let mut info = NutritionalInfo::default();
            for n in &food.food_nutrients {
                match n.id {
                    1008 | 2047 | 2048 | 1001 => {
                        if info.calories == 0.0 {
                            info.calories = n.value;
                        }
                    }
                    1003 => info.protein = n.value,
                    1005 => info.carbohydrates = n.value,
                    1004 => info.fat = n.value,
                    _ => {}
                }
            }

            cache.ingredients.insert(name.to_string(), info.clone());
            let _ = self.save_cache(&cache);
            Ok(info)
        } else {
            Err(AppError::NotFound(format!(
                "Ingredient '{}' not found",
                name
            )))
        }
    }

    pub async fn get_plan_nutrition(&self, plan_id: &str) -> AppResult<PlanNutrition> {
        let mut cache = self.load_cache();
        if let Some(plan_nutrition) = cache.plans.get(plan_id) {
            info!("Found nutrition for plan '{}' in local cache", plan_id);
            return Ok(plan_nutrition.clone());
        }

        let items_to_process =
            if let Ok(Some(shopping_list)) = self.shopping_repo.get_by_plan_id(plan_id) {
                shopping_list
                    .items
                    .into_iter()
                    .map(|i| (i.name, i.quantity))
                    .collect::<Vec<_>>()
            } else {
                let plan_idx = self.plan_repo.get_index_by_id(plan_id)?;
                plan_idx
                    .proteinas
                    .into_iter()
                    .map(|p| (p, None))
                    .collect::<Vec<_>>()
            };

        let mut nutrition = PlanNutrition {
            plan_id: plan_id.to_string(),
            ..Default::default()
        };

        for (name, qty) in items_to_process {
            if let Ok(info) = self.get_nutrition_internal(&mut cache, &name).await {
                let multiplier = qty.map(|q| self.parse_multiplier(&q)).unwrap_or(1.0);
                let scaled = NutritionalInfo {
                    calories: info.calories * multiplier,
                    protein: info.protein * multiplier,
                    carbohydrates: info.carbohydrates * multiplier,
                    fat: info.fat * multiplier,
                };

                nutrition.total_calories += scaled.calories;
                nutrition.total_protein += scaled.protein;
                nutrition.total_carbs += scaled.carbohydrates;
                nutrition.total_fat += scaled.fat;
                nutrition.breakdown_by_item.insert(name, scaled);
            }
        }

        cache.plans.insert(plan_id.to_string(), nutrition.clone());
        let _ = self.save_cache(&cache);

        Ok(nutrition)
    }

    fn parse_multiplier(&self, qty_str: &str) -> f32 {
        let qty_str = qty_str.to_lowercase();
        if qty_str.contains("kg") {
            qty_str
                .replace("kg", "")
                .trim()
                .parse::<f32>()
                .unwrap_or(1.0)
                * 10.0
        } else if qty_str.contains('g') {
            qty_str
                .replace('g', "")
                .trim()
                .parse::<f32>()
                .unwrap_or(100.0)
                / 100.0
        } else {
            1.0
        }
    }
}
