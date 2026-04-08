use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlanMetadata {
    pub plan_id: String,
    #[serde(default)]
    pub display_name: Option<String>,
    pub is_favorite: bool,
    pub rating: Option<u8>,
    pub notes: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanIndex {
    pub id: String,
    #[serde(default)]
    pub fecha: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    pub proteinas: Vec<String>,
    pub enviado: bool,
    #[serde(default)]
    pub is_favorite: bool,
    #[serde(default)]
    pub rating: Option<u8>,
    #[serde(default)]
    pub weekly_structure: Option<Vec<WeeklyMealInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyMealInfo {
    pub day_index: u8,
    pub meal_type: String,
    pub description: Option<String>,
    #[serde(default)]
    pub day_id: Option<String>,
    #[serde(default)]
    pub recipe_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredPlan {
    pub title: String,
    pub instructions: Option<String>,
    pub days: Vec<StructuredDay>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDay {
    pub day_id: String,
    pub day_index: u8,
    pub label: String,
    pub recipes: Vec<StructuredRecipe>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredRecipe {
    pub recipe_id: String,
    pub meal_type: MealType,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSuggestion {
    pub plan_id: String,
    pub day_id: String,
    pub recipe_id: String,
    pub original_recipe: StructuredRecipe,
    pub suggested_recipe: StructuredRecipe,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanDetail {
    pub id: String,
    pub markdown_content: String,
    #[serde(default)]
    pub structured_plan: Option<StructuredPlan>,
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
    pub auto_generate_plan: bool,
    pub cron_expression: String,
    pub default_meal_type: MealType,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SyncStatus {
    Idle,
    Syncing,
    Success,
    Error(String),
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum MealType {
    #[default]
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

impl MealType {
    pub fn display_name(&self) -> &'static str {
        match self {
            MealType::Breakfast => "Desayuno",
            MealType::Lunch => "Almuerzo",
            MealType::Dinner => "Cena",
            MealType::Snack => "Snack",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            MealType::Breakfast => "breakfast",
            MealType::Lunch => "lunch",
            MealType::Dinner => "dinner",
            MealType::Snack => "snack",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEntry {
    pub date: String,
    pub meal_type: MealType,
    pub plan_id: String,
    #[serde(default)]
    pub assignment_id: Option<String>,
    #[serde(default)]
    pub day_id: Option<String>,
    #[serde(default)]
    pub plan_day_index: Option<u8>,
    #[serde(default)]
    pub recipe_id: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum VariationType {
    Vegan,
    Keto,
    GlutenFree,
    LowCarb,
    HighProtein,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilters {
    pub query: Option<String>,
    pub only_favorites: bool,
    pub min_rating: Option<u8>,
    pub protein_contains: Option<String>,
}

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
    pub water: std::collections::HashMap<String, WaterRecord>,
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

// Internal arguments for invoke and fallback
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanIdArgs {
    pub plan_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdArgs {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRatingArgs {
    pub plan_id: String,
    pub rating: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNoteArgs {
    pub plan_id: String,
    pub note: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayNameArgs {
    pub plan_id: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveConfigArgs {
    pub config: AppConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveExcludedArgs {
    pub ingredients: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleExclusionArgs {
    pub ingredient: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleItemArgs {
    pub plan_id: String,
    pub item_name: String,
    pub checked: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailArgs {
    pub plan_id: String,
    pub target_email: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanTagArgs {
    pub plan_id: String,
    pub tag_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariationArgs {
    pub plan_id: String,
    pub variation: VariationType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArgs {
    pub filters: SearchFilters,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagArgs {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PantryItemArgs {
    pub item: PantryItem,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportDataArgs {
    pub backup: AppBackup,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavePreferencesArgs {
    pub preferences: UIPreferences,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWaterArgs {
    pub date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWaterArgs {
    pub date: String,
    pub current: f32,
    pub target: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWaterHistoryArgs {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRangeArgs {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignPlanArgs {
    pub date: String,
    pub meal_type: MealType,
    pub plan_id: String,
    pub recipe_id: Option<String>,
    pub plan_day_index: Option<u8>,
    pub assignment_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignWeeklyPlanArgs {
    pub start_date: String,
    pub plan_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEntryArgs {
    pub date: String,
    pub meal_type: MealType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSuggestionArgs {
    pub plan_id: String,
    pub day_id: String,
    pub recipe_id: String,
    pub prompt: String,
}
