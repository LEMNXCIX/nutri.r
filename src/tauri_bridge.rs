use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlanMetadata {
    pub plan_id: String,
    pub is_favorite: bool,
    pub rating: Option<u8>,
    pub notes: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanIndex {
    pub id: String,
    pub fecha: String,
    pub proteinas: Vec<String>,
    pub enviado: bool,
    #[serde(default)]
    pub is_favorite: bool,
    #[serde(default)]
    pub rating: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanDetail {
    pub id: String,
    pub markdown_content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub smtp_to: String,
    pub prompt_maestro: String,
    pub ollama_model: String,
    pub ollama_url: String,
    pub usda_api_key: String,
    pub sync_server_url: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UIPreferences {
    pub theme: String,
    pub primary_color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExcludedIngredients {
    pub ingredients: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IngredientStats {
    pub name: String,
    pub count: usize,
    pub is_excluded: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WaterRecord {
    pub current: f32,
    pub target: f32,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShoppingItem {
    pub name: String,
    pub category: String,
    pub quantity: Option<String>,
    #[serde(default)]
    pub checked: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingList {
    pub id: String,
    pub plan_id: String,
    pub created_at: String,
    pub items: Vec<ShoppingItem>,
}

// Internal arguments for invoke
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlanIdArgs<'a> {
    pub plan_id: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IdArgs<'a> {
    id: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SetRatingArgs<'a> {
    pub plan_id: &'a str,
    pub rating: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SetNoteArgs<'a> {
    pub plan_id: &'a str,
    pub note: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveConfigArgs {
    config: AppConfig,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveExcludedArgs {
    ingredients: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ToggleExclusionArgs {
    ingredient: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ToggleItemArgs<'a> {
    pub plan_id: &'a str,
    pub item_name: &'a str,
    pub checked: bool,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window","__TAURI__","core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub async fn perform_sync() -> Result<String, String> {
    match invoke("perform_sync", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Sincronización falló: respuesta inesperada".to_string()),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn pull_from_server() -> Result<String, String> {
    match invoke("pull_from_server", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Pull falló: respuesta inesperada".to_string()),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn push_to_server() -> Result<String, String> {
    match invoke("push_to_server", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Push falló: respuesta inesperada".to_string()),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn generate_week() -> Result<String, String> {
    match invoke("generate_week", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Failed to generate week: response not a string".to_string()),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_index() -> Result<Vec<PlanIndex>, String> {
    match invoke("get_index", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(index) => Ok(index),
            Err(e) => Err(format!("Failed to parse index: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_plan_content(id: &str) -> Result<String, String> {
    let args = IdArgs { id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("get_plan_content", args_js).await {
        Ok(response) => match response.as_string() {
            Some(content) => Ok(content),
            None => Err("Failed to get plan content: response not a string".to_string()),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_config() -> Result<AppConfig, String> {
    match invoke("get_config", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(config) => Ok(config),
            Err(e) => Err(format!("Failed to parse config: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn is_mobile() -> bool {
    // Return true if tauri::is_mobile() returns true or if we are not in tauri (fallback)
    match invoke("is_mobile", JsValue::NULL).await {
        Ok(response) => response.as_bool().unwrap_or(false),
        Err(_) => false,
    }
}

pub async fn save_config(config: AppConfig) -> Result<(), String> {
    let args = SaveConfigArgs { config };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("save_config", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn list_ollama_models() -> Result<Vec<OllamaModel>, String> {
    match invoke("list_ollama_models", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(models) => Ok(models),
            Err(e) => Err(format!("Failed to parse models: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_excluded_ingredients() -> Result<Vec<String>, String> {
    match invoke("get_excluded_ingredients", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(ingredients) => Ok(ingredients),
            Err(e) => Err(format!("Failed to parse excluded ingredients: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_ingredient_stats() -> Result<Vec<IngredientStats>, String> {
    match invoke("get_ingredient_stats", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(stats) => Ok(stats),
            Err(e) => Err(format!("Failed to parse stats: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn toggle_ingredient_exclusion(ingredient: String) -> Result<Vec<String>, String> {
    let args = ToggleExclusionArgs { ingredient };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("toggle_ingredient_exclusion", args_js).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(ingredients) => Ok(ingredients),
            Err(e) => Err(format!("Failed to parse ingredients: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn save_excluded_ingredients(ingredients: Vec<String>) -> Result<(), String> {
    let args = SaveExcludedArgs { ingredients };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("save_excluded_ingredients", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn toggle_favorite(plan_id: &str) -> Result<bool, String> {
    let args = PlanIdArgs { plan_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("toggle_favorite", args_js).await {
        Ok(response) => match response.as_bool() {
            Some(fav) => Ok(fav),
            None => Err("Failed to toggle favorite: response not a bool".to_string()),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_plan_metadata(plan_id: &str) -> Result<PlanMetadata, String> {
    let args = PlanIdArgs { plan_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("get_plan_metadata", args_js).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(metadata) => Ok(metadata),
            Err(e) => Err(format!("Failed to parse metadata: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_favorite_plans() -> Result<Vec<PlanIndex>, String> {
    match invoke("get_favorite_plans", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(plans) => Ok(plans),
            Err(e) => Err(format!("Failed to parse favorite plans: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn set_plan_rating(plan_id: &str, rating: u8) -> Result<(), String> {
    let args = SetRatingArgs { plan_id, rating };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("set_plan_rating", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn set_plan_note(plan_id: &str, note: String) -> Result<(), String> {
    let args = SetNoteArgs { plan_id, note };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("set_plan_note", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn generate_shopping_list(plan_id: &str) -> Result<ShoppingList, String> {
    let args = PlanIdArgs { plan_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("generate_shopping_list", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn get_shopping_list(plan_id: &str) -> Result<Option<ShoppingList>, String> {
    let args = PlanIdArgs { plan_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("get_shopping_list", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn toggle_shopping_item(
    plan_id: &str,
    item_name: &str,
    checked: bool,
) -> Result<(), String> {
    let args = ToggleItemArgs {
        plan_id,
        item_name,
        checked,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("toggle_shopping_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub enum MealType {
    #[default]
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEntry {
    pub date: String,
    pub meal_type: MealType,
    pub plan_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssignPlanArgs {
    pub date: String,
    pub meal_type: MealType,
    pub plan_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetRangeArgs {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RemoveEntryArgs {
    pub date: String,
    pub meal_type: MealType,
}

pub async fn assign_plan_to_date(
    date: String,
    meal_type: MealType,
    plan_id: String,
) -> Result<(), String> {
    let args = AssignPlanArgs {
        date,
        meal_type,
        plan_id,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("assign_plan_to_date", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_calendar_range(
    start_date: String,
    end_date: String,
) -> Result<Vec<CalendarEntry>, String> {
    let args = GetRangeArgs {
        start_date,
        end_date,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("get_calendar_range", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn remove_calendar_entry(date: String, meal_type: MealType) -> Result<(), String> {
    let args = RemoveEntryArgs { date, meal_type };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("remove_calendar_entry", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub total_plans: usize,
    pub favorite_plans: usize,
    pub recipes_count: usize,
    pub ingredients_count: usize,
    pub meal_distribution: std::collections::HashMap<String, usize>,
    pub monthly_activity: Vec<MonthlyData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyData {
    pub month: String,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IngredientTrend {
    pub name: String,
    pub count: usize,
}

pub async fn get_statistics() -> Result<Statistics, String> {
    match invoke("get_statistics", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(stats) => Ok(stats),
            Err(e) => Err(format!("Failed to parse statistics: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_ingredient_trends() -> Result<Vec<IngredientTrend>, String> {
    match invoke("get_ingredient_trends", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(trends) => Ok(trends),
            Err(e) => Err(format!("Failed to parse trends: {}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NutritionalInfo {
    pub calories: f32,
    pub protein: f32,
    pub carbohydrates: f32,
    pub fat: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlanNutrition {
    pub plan_id: String,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
    pub breakdown_by_item: std::collections::HashMap<String, NutritionalInfo>,
}

pub async fn calculate_nutrition(plan_id: &str) -> Result<PlanNutrition, String> {
    let args = PlanIdArgs { plan_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("calculate_nutrition", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum VariationType {
    Vegan,
    Keto,
    GlutenFree,
    LowCarb,
    HighProtein,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VariationArgs<'a> {
    pub plan_id: &'a str,
    pub variation: VariationType,
}

pub async fn generate_variation(plan_id: &str, variation: VariationType) -> Result<String, String> {
    let args = VariationArgs { plan_id, variation };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("generate_variation", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilters {
    pub query: Option<String>,
    pub only_favorites: bool,
    pub min_rating: Option<u8>,
    pub protein_contains: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchArgs {
    filters: SearchFilters,
}

pub async fn search_plans(filters: SearchFilters) -> Result<Vec<PlanIndex>, String> {
    let args = SearchArgs { filters };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("search_plans", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateTagArgs {
    pub name: String,
    pub color: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlanTagArgs {
    pub plan_id: String,
    pub tag_id: String,
}

pub async fn get_all_tags() -> Result<Vec<Tag>, String> {
    let result = invoke("get_all_tags", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn create_tag(name: String, color: String) -> Result<Tag, String> {
    let args = CreateTagArgs { name, color };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("create_tag", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn add_tag_to_plan(plan_id: String, tag_id: String) -> Result<(), String> {
    let args = PlanTagArgs { plan_id, tag_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("add_tag_to_plan", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn remove_tag_from_plan(plan_id: String, tag_id: String) -> Result<(), String> {
    let args = PlanTagArgs { plan_id, tag_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("remove_tag_from_plan", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

// Pantry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PantryItem {
    pub id: String,
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    pub expiration_date: Option<String>,
    pub category: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppBackup {
    pub version: String,
    pub last_updated: String,
    pub config: AppConfig,
    pub plans: Vec<PlanIndex>,
    pub plan_details: Vec<PlanDetail>,
    pub metadata: Vec<PlanMetadata>,
    pub tags: Vec<Tag>,
    pub calendar: Vec<CalendarEntry>,
    pub pantry: Vec<PantryItem>,
    pub excluded_ingredients: ExcludedIngredients,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportDataArgs {
    pub backup: AppBackup,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PantryItemArgs {
    pub item: PantryItem,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SavePreferencesArgs {
    pub preferences: UIPreferences,
}

pub async fn get_pantry_items() -> Result<Vec<PantryItem>, String> {
    let result = invoke("get_pantry_items", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn add_pantry_item(item: PantryItem) -> Result<(), String> {
    let args = PantryItemArgs { item };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("add_pantry_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn update_pantry_item(item: PantryItem) -> Result<(), String> {
    let args = PantryItemArgs { item };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("update_pantry_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn delete_pantry_item(id: String) -> Result<(), String> {
    let args = IdArgs { id: &id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("delete_pantry_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn export_data() -> Result<AppBackup, String> {
    match invoke("export_data", JsValue::NULL).await {
        Ok(res) => serde_wasm_bindgen::from_value(res).map_err(|e| e.to_string()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn import_data(backup: AppBackup) -> Result<(), String> {
    let args = ImportDataArgs { backup };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("import_data", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_ui_preferences() -> Result<UIPreferences, String> {
    let result = invoke("get_ui_preferences", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn save_ui_preferences(preferences: UIPreferences) -> Result<(), String> {
    let args = SavePreferencesArgs { preferences };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("save_ui_preferences", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_achievements() -> Result<Vec<Achievement>, String> {
    let result = invoke("get_achievements", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendEmailArgs {
    pub plan_id: String,
    pub target_email: String,
}

pub async fn send_plan_email(plan_id: String, target_email: String) -> Result<(), String> {
    let args = SendEmailArgs {
        plan_id,
        target_email,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;

    match invoke("send_plan_email", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

// Water tracking
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetWaterArgs {
    pub date: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateWaterArgs {
    pub date: String,
    pub current: f32,
    pub target: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetWaterHistoryArgs {
    pub start_date: String,
    pub end_date: String,
}

pub async fn get_water_intake(date: String) -> Result<WaterRecord, String> {
    let args = GetWaterArgs { date };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("get_water_intake", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}

pub async fn update_water_intake(date: String, current: f32, target: f32) -> Result<(), String> {
    let args = UpdateWaterArgs {
        date,
        current,
        target,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("update_water_intake", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_water_history(
    start_date: String,
    end_date: String,
) -> Result<std::collections::HashMap<String, WaterRecord>, String> {
    let args = GetWaterHistoryArgs {
        start_date,
        end_date,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = invoke("get_water_history", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| format!("{:?}", e))?)
        .map_err(|e| e.to_string())
}
