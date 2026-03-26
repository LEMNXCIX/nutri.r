use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

static DEBUG_LOGS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
static IS_API_ONLINE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

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
struct PlanIdArgs {
    pub plan_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IdArgs {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetRatingArgs {
    pub plan_id: String,
    pub rating: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetNoteArgs {
    pub plan_id: String,
    pub note: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetDisplayNameArgs {
    pub plan_id: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveConfigArgs {
    pub config: AppConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveExcludedArgs {
    pub ingredients: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToggleExclusionArgs {
    pub ingredient: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToggleItemArgs {
    pub plan_id: String,
    pub item_name: String,
    pub checked: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendEmailArgs {
    pub plan_id: String,
    pub target_email: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlanTagArgs {
    pub plan_id: String,
    pub tag_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VariationArgs {
    pub plan_id: String,
    pub variation: VariationType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchArgs {
    pub filters: SearchFilters,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateTagArgs {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PantryItemArgs {
    pub item: PantryItem,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportDataArgs {
    pub backup: AppBackup,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SavePreferencesArgs {
    pub preferences: UIPreferences,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetWaterArgs {
    pub date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateWaterArgs {
    pub date: String,
    pub current: f32,
    pub target: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetWaterHistoryArgs {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetRangeArgs {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssignPlanArgs {
    pub date: String,
    pub meal_type: MealType,
    pub plan_id: String,
    pub recipe_id: Option<String>,
    pub plan_day_index: Option<u8>,
    pub assignment_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssignWeeklyPlanArgs {
    pub start_date: String,
    pub plan_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoveEntryArgs {
    pub date: String,
    pub meal_type: MealType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecipeSuggestionArgs {
    pub plan_id: String,
    pub recipe_id: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplyRecipeEditArgs {
    pub plan_id: String,
    pub recipe_id: String,
    pub recipe: StructuredRecipe,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwapEntriesArgs {
    pub first_date: String,
    pub first_meal_type: MealType,
    pub second_date: String,
    pub second_meal_type: MealType,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window","__TAURI__","core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub fn is_tauri() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(tauri) = js_sys::Reflect::get(&window, &"__TAURI__".into()) {
                return !tauri.is_undefined() && !tauri.is_null();
            }
        }
    }
    false
}

pub fn log_trace(msg: String) {
    if let Ok(mut logs) = DEBUG_LOGS.lock() {
        let logs: &mut Vec<String> = &mut *logs;
        logs.push(msg.clone());
        if logs.len() > 100 {
            logs.remove(0);
        }
    }

    // Diagnostic console log for the developer
    web_sys::console::debug_1(&JsValue::from_str(&format!("[DEBUG_LOG] {}", msg)));

    if let Some(window) = web_sys::window() {
        let init = web_sys::CustomEventInit::new();
        init.set_detail(&JsValue::from_str(&msg));
        init.set_bubbles(true);
        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("debug-log", &init) {
            let _ = window.dispatch_event(&event);
        }
    }
    log::info!("[TRACE] {}", msg);
}

pub fn get_debug_logs() -> Vec<String> {
    if let Ok(logs) = DEBUG_LOGS.lock() {
        let logs: &Vec<String> = &*logs;
        logs.clone()
    } else {
        Vec::new()
    }
}

pub fn clear_debug_logs() {
    if let Ok(mut logs) = DEBUG_LOGS.lock() {
        let logs: &mut Vec<String> = &mut *logs;
        logs.clear();
    }
}

async fn call_api_fallback(cmd: &str, args: JsValue, api_base: &str) -> Result<JsValue, String> {
    let client = reqwest::Client::new();
    log::info!(
        "Fallback API: Ejecutando comando '{}' contra '{}'",
        cmd,
        api_base
    );

    match cmd {
        "get_config" => {
            let res = client
                .get(format!("{}/config", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let config = res.json::<AppConfig>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&config).map_err(|e| e.to_string())
        }
        "save_config" => {
            let save_args: SaveConfigArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/config", api_base))
                .json(&save_args.config)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "get_index" => {
            let res = client
                .get(format!("{}/plans", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let index = res
                .json::<Vec<PlanIndex>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&index).map_err(|e| e.to_string())
        }
        "get_plan_content" => {
            let id_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/plans/{}", api_base, id_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let content = res.json::<String>().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_str(&content))
        }
        "get_plan_detail" => {
            let id_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/plans/{}/detail", api_base, id_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let detail = res.json::<PlanDetail>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&detail).map_err(|e| e.to_string())
        }
        "delete_plan" => {
            let id_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .delete(format!("{}/plans/{}", api_base, id_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "generate_week" => {
            let res = client
                .post(format!("{}/plans/generate", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let id = res.json::<String>().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_str(&id))
        }
        "get_favorite_plans" => {
            let res = client
                .get(format!("{}/plans/favorites", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let plans = res
                .json::<Vec<PlanIndex>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&plans).map_err(|e| e.to_string())
        }
        "toggle_favorite" => {
            let fav_args: PlanIdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .post(format!("{}/plans/{}/favorite", api_base, fav_args.plan_id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let is_fav = res.json::<bool>().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_bool(is_fav))
        }
        "get_plan_metadata" => {
            let meta_args: PlanIdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/plans/{}/metadata", api_base, meta_args.plan_id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let meta = res
                .json::<PlanMetadata>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&meta).map_err(|e| e.to_string())
        }
        "set_plan_rating" => {
            let rate_args: SetRatingArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/plans/{}/rating", api_base, rate_args.plan_id))
                .json(&rate_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "set_plan_note" => {
            let note_args: SetNoteArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/plans/{}/note", api_base, note_args.plan_id))
                .json(&note_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "set_plan_display_name" => {
            let display_name_args: SetDisplayNameArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!(
                    "{}/plans/{}/display-name",
                    api_base, display_name_args.plan_id
                ))
                .json(&display_name_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "generate_shopping_list" => {
            let shop_args: PlanIdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .post(format!("{}/shopping/{}", api_base, shop_args.plan_id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let list = res
                .json::<ShoppingList>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&list).map_err(|e| e.to_string())
        }
        "get_shopping_list" => {
            let shop_args: PlanIdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/shopping/{}", api_base, shop_args.plan_id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let list = res
                .json::<Option<ShoppingList>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&list).map_err(|e| e.to_string())
        }
        "toggle_shopping_item" => {
            let item_args: ToggleItemArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!(
                    "{}/shopping/{}/toggle",
                    api_base, item_args.plan_id
                ))
                .json(&item_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "get_calendar_range" => {
            let range_args: GetRangeArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let url = format!(
                "{}/calendar?startDate={}&endDate={}",
                api_base, range_args.start_date, range_args.end_date
            );
            let res = client.get(url).send().await.map_err(|e| e.to_string())?;
            let entries = res
                .json::<Vec<CalendarEntry>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&entries).map_err(|e| e.to_string())
        }
        "assign_plan_to_date" => {
            let assign_args: AssignPlanArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/calendar", api_base))
                .json(&assign_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "assign_weekly_plan_to_date" => {
            let assign_args: AssignWeeklyPlanArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/calendar/weekly", api_base))
                .json(&assign_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "remove_calendar_entry" => {
            let rem_args: RemoveEntryArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .delete(format!("{}/calendar", api_base))
                .json(&rem_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "swap_calendar_entries" => {
            let swap_args: SwapEntriesArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/calendar/swap", api_base))
                .json(&swap_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "get_water_intake" => {
            let water_args: GetWaterArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/water/{}", api_base, water_args.date))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let record = res.json::<WaterRecord>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&record).map_err(|e| e.to_string())
        }
        "update_water_intake" => {
            let water_args: UpdateWaterArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/water/{}", api_base, water_args.date))
                .json(&water_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "get_water_history" => {
            let hist_args: GetWaterHistoryArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let url = format!(
                "{}/water/history?startDate={}&endDate={}",
                api_base, hist_args.start_date, hist_args.end_date
            );
            let res = client.get(url).send().await.map_err(|e| e.to_string())?;
            let history = res
                .json::<std::collections::HashMap<String, WaterRecord>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&history).map_err(|e| e.to_string())
        }
        "get_statistics" => {
            let res = client
                .get(format!("{}/stats", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let stats = res.json::<Statistics>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&stats).map_err(|e| e.to_string())
        }
        "get_ingredient_trends" => {
            let res = client
                .get(format!("{}/stats/trends", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let trends = res
                .json::<Vec<IngredientTrend>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&trends).map_err(|e| e.to_string())
        }
        "calculate_nutrition" => {
            let nut_args: PlanIdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/plans/{}/nutrition", api_base, nut_args.plan_id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let nutrition = res
                .json::<PlanNutrition>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&nutrition).map_err(|e| e.to_string())
        }
        "generate_variation" => {
            let var_args: VariationArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .post(format!("{}/plans/{}/variation", api_base, var_args.plan_id))
                .json(&var_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let content = res.json::<String>().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_str(&content))
        }
        "preview_recipe_edit" => {
            let edit_args: RecipeSuggestionArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .post(format!(
                    "{}/plans/{}/recipes/{}/suggestion",
                    api_base, edit_args.plan_id, edit_args.recipe_id
                ))
                .json(&serde_json::json!({ "prompt": edit_args.prompt }))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let suggestion = res
                .json::<RecipeSuggestion>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&suggestion).map_err(|e| e.to_string())
        }
        "apply_recipe_edit" => {
            let edit_args: ApplyRecipeEditArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .patch(format!(
                    "{}/plans/{}/recipes/{}",
                    api_base, edit_args.plan_id, edit_args.recipe_id
                ))
                .json(&serde_json::json!({ "recipe": edit_args.recipe }))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let detail = res.json::<PlanDetail>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&detail).map_err(|e| e.to_string())
        }
        "search_plans" => {
            let search_args: SearchArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .post(format!("{}/plans/search", api_base))
                .json(&search_args.filters)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let plans = res
                .json::<Vec<PlanIndex>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&plans).map_err(|e| e.to_string())
        }
        "get_all_tags" => {
            let res = client
                .get(format!("{}/tags", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let tags = res.json::<Vec<Tag>>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&tags).map_err(|e| e.to_string())
        }
        "create_tag" => {
            let tag_args: CreateTagArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .post(format!("{}/tags", api_base))
                .json(&tag_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let tag = res.json::<Tag>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&tag).map_err(|e| e.to_string())
        }
        "delete_tag" => {
            let tag_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .delete(format!("{}/tags/{}", api_base, tag_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "add_tag_to_plan" => {
            let tag_args: PlanTagArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!(
                    "{}/tags/{}/{}",
                    api_base, tag_args.plan_id, tag_args.tag_id
                ))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "remove_tag_from_plan" => {
            let tag_args: PlanTagArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .delete(format!(
                    "{}/tags/{}/{}",
                    api_base, tag_args.plan_id, tag_args.tag_id
                ))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "get_pantry_items" => {
            let res = client
                .get(format!("{}/pantry", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let items = res
                .json::<Vec<PantryItem>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&items).map_err(|e| e.to_string())
        }
        "add_pantry_item" => {
            let item_args: PantryItemArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/pantry", api_base))
                .json(&item_args.item)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "update_pantry_item" => {
            let item_args: PantryItemArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .put(format!("{}/pantry", api_base))
                .json(&item_args.item)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "delete_pantry_item" => {
            let id_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .delete(format!("{}/pantry/{}", api_base, id_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "export_data" => {
            let res = client
                .get(format!("{}/backup/export", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let backup = res.json::<AppBackup>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&backup).map_err(|e| e.to_string())
        }
        "import_data" => {
            let import_args: ImportDataArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/backup/import", api_base))
                .json(&import_args.backup)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "get_achievements" => {
            let res = client
                .get(format!("{}/achievements", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let achievements = res
                .json::<Vec<Achievement>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&achievements).map_err(|e| e.to_string())
        }
        "get_ui_preferences" => {
            let res = client
                .get(format!("{}/preferences", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let prefs = res
                .json::<UIPreferences>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&prefs).map_err(|e| e.to_string())
        }
        "save_ui_preferences" => {
            let pref_args: SavePreferencesArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/preferences", api_base))
                .json(&pref_args.preferences)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "send_plan_email" => {
            let email_args: SendEmailArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/email/send", api_base))
                .json(&email_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "perform_sync" => {
            let res = client
                .post(format!("{}/sync", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = res.json::<String>().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_str(&status))
        }
        "pull_from_server" => {
            let res = client
                .post(format!("{}/sync/pull", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = res.json::<String>().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_str(&status))
        }
        "push_to_server" => {
            let res = client
                .post(format!("{}/sync/push", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = res.json::<String>().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_str(&status))
        }
        "get_sync_status" => {
            let res = client
                .get(format!("{}/sync/status", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let status = res.json::<SyncStatus>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&status).map_err(|e| e.to_string())
        }
        "get_excluded_ingredients" => {
            let res = client
                .get(format!("{}/ingredients/excluded", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let ingredients = res.json::<Vec<String>>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&ingredients).map_err(|e| e.to_string())
        }
        "save_excluded_ingredients" => {
            let excl_args: SaveExcludedArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/ingredients/excluded", api_base))
                .json(&excl_args.ingredients)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "get_ingredient_stats" => {
            let res = client
                .get(format!("{}/ingredients/stats", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let stats = res
                .json::<Vec<IngredientStats>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&stats).map_err(|e| e.to_string())
        }
        "toggle_ingredient_exclusion" => {
            let excl_args: ToggleExclusionArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .post(format!("{}/ingredients/toggle", api_base))
                .json(&excl_args)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let ingredients = res.json::<Vec<String>>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&ingredients).map_err(|e| e.to_string())
        }
        "list_ollama_models" => {
            let res = client
                .get(format!("{}/ollama/models", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let models = res
                .json::<Vec<OllamaModel>>()
                .await
                .map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&models).map_err(|e| e.to_string())
        }
        _ => {
            let err_msg = format!("Command '{}' not implemented for web fallback", cmd);
            log::warn!("{}", err_msg);
            Err(err_msg)
        }
    }
}

pub async fn check_endpoint(url: &str) -> Result<(), String> {
    log_trace(format!("CHECK_ENDPOINT: {}", url));
    let client = reqwest::Client::builder()
        .build()
        .map_err(|e: reqwest::Error| e.to_string())?;

    match client.get(url).send().await {
        Ok(res) => {
            let status = res.status();
            log_trace(format!("CHECK_RES: {} -> {}", url, status));
            if status.is_success() {
                Ok(())
            } else {
                Err(format!(
                    "Servidor alcanzado pero devolvió error: {}",
                    status
                ))
            }
        }
        Err(e) => {
            let e: reqwest::Error = e;
            let msg = e.to_string();
            log_trace(format!("CHECK_ERR: {} -> {}", url, msg));
            if msg.contains("Failed to fetch") || msg.contains("NetworkError") {
                Err("No se pudo conectar al servidor. Verifica la IP, el puerto y que no haya un firewall bloqueando la conexión.".to_string())
            } else {
                Err(format!("Error de red: {}", msg))
            }
        }
    }
}

async fn get_api_base_url() -> String {
    if is_tauri() {
        if let Ok(config_js) = invoke("get_config", JsValue::NULL).await {
            if let Ok(config) = serde_wasm_bindgen::from_value::<AppConfig>(config_js) {
                if !config.sync_server_url.is_empty() {
                    let mut base = config.sync_server_url.trim_end_matches('/').to_string();
                    if base.ends_with("/api/sync") {
                        base = base.replace("/api/sync", "/api");
                    } else if !base.contains("/api") {
                        base = format!("{}/api", base);
                    }
                    return base;
                }
            }
        }
    }
    // // Web
    // if let Some(window) = web_sys::window() {
    //     if let Ok(origin) = window.location().origin() {
    //         if !origin.is_empty() && origin != "null" {
    //             return format!("{}/api", origin);
    //         }
    //     }
    // }
    "http://localhost:3001/api".to_string()
}

pub async fn check_health() -> bool {
    let api_base = get_api_base_url().await;
    let url = format!("{}/health", api_base);
    log_trace(format!("HEALTH_CHECK: {}", url));

    let client = reqwest::Client::new();

    let is_online = match client.get(&url).send().await {
        Ok(res) => {
            let ok = res.status().is_success();
            log_trace(format!("HEALTH_RES: {} -> {}", url, ok));
            ok
        }
        Err(e) => {
            log_trace(format!("HEALTH_ERR: {} -> {}", url, e));
            false
        }
    };

    if let Ok(mut health) = IS_API_ONLINE.lock() {
        *health = is_online;
    }

    is_online
}

pub async fn start_health_check_loop() {
    log_trace("NET: Starting health check background loop".to_string());
    leptos::task::spawn_local(async {
        loop {
            // Check health every 30 seconds
            let _ = check_health().await;
            gloo_timers::future::sleep(std::time::Duration::from_secs(30)).await;
        }
    });
}

pub async fn auto_pull() {
    log_trace("SYNC: Auto Pull Starting...".to_string());
    match safe_invoke("pull_from_server", JsValue::NULL).await {
        Ok(_) => log_trace("SYNC: Auto Pull OK".to_string()),
        Err(e) => log_trace(format!("SYNC: Auto Pull Failed: {}", e)),
    }
}

pub async fn auto_push() {
    run_auto_push().await;
}

async fn run_auto_push() {
    log_trace("SYNC: Auto Push Starting...".to_string());
    match safe_invoke("push_to_server", JsValue::NULL).await {
        Ok(_) => log_trace("SYNC: Auto Push OK".to_string()),
        Err(e) => log_trace(format!("SYNC: Auto Push Failed: {}", e)),
    }
}

fn schedule_auto_push() {
    leptos::task::spawn_local(run_auto_push());
}

async fn safe_invoke(cmd: &str, args: JsValue) -> Result<JsValue, String> {
    let mut use_tauri = is_tauri();
    let is_config_cmd = cmd == "get_config" || cmd == "save_config" || cmd == "is_mobile";
    let is_sync_cmd = cmd == "pull_from_server"
        || cmd == "push_to_server"
        || cmd == "get_sync_status"
        || cmd == "perform_sync";

    // Detect write operations for auto-sync
    let is_write = !is_sync_cmd
        && (cmd.starts_with("save_")
            || cmd.starts_with("add_")
            || cmd.starts_with("delete_")
            || cmd.starts_with("toggle_")
            || cmd.starts_with("assign_")
            || cmd.starts_with("set_")
            || cmd.starts_with("import_")
            || cmd.starts_with("remove_")
            || cmd.starts_with("update_")
            || cmd.starts_with("generate_")
            || cmd.starts_with("clear_"));

    // If we are in Tauri (Desktop/Mobile), verify if we are in mobile to force API
    if use_tauri && !is_config_cmd {
        match invoke("is_mobile", JsValue::NULL).await {
            Ok(js_val) => {
                if js_val.as_bool().unwrap_or(false) {
                    use_tauri = false;
                }
            }
            Err(_) => {}
        }
    }

    log_trace(format!("SAFE_INVOKE: {} (T_PREV: {})", cmd, use_tauri));

    let res = if use_tauri {
        let res = invoke(cmd, args).await.map_err(|e| format!("{:?}", e));
        match &res {
            Ok(_) => log_trace(format!("T_RES: {} -> OK", cmd)),
            Err(e) => log_trace(format!("T_RES: {} -> ERR: {}", cmd, e)),
        }
        res
    } else {
        let api_base = get_api_base_url().await;

        // Use cached health status
        let is_online = if let Ok(health) = IS_API_ONLINE.lock() {
            *health
        } else {
            true
        };

        if !is_config_cmd && !is_sync_cmd && !is_online {
            log_trace(format!(
                "FALLBACK: API marked as OFFLINE, skipping health check for {}",
                cmd
            ));
            return invoke(cmd, args).await.map_err(|e| format!("{:?}", e));
        }

        log_trace(format!("REQ: {} a {}", cmd, api_base));
        let res = call_api_fallback(cmd, args.clone(), &api_base).await;

        // Update health status based on result
        match &res {
            Ok(_) => {
                log_trace(format!("RES: {} -> OK", cmd));
                if let Ok(mut health) = IS_API_ONLINE.lock() {
                    *health = true;
                }
            }
            Err(e) => {
                log_trace(format!("RES: {} -> ERR: {}", cmd, e));
                if e.contains("NetworkError") || e.contains("Failed to fetch") {
                    if let Ok(mut health) = IS_API_ONLINE.lock() {
                        *health = false;
                    }
                    log_trace(format!(
                        "RETRY_LOCAL: API error {}, attempting LOCAL fallback",
                        e
                    ));
                    return invoke(cmd, args).await.map_err(|err| format!("{:?}", err));
                }
            }
        }
        res
    };

    // Unified auto_push for all successful write operations
    if res.is_ok() && is_write {
        schedule_auto_push();
    }

    res
}

fn emit_toast(message: &str, is_error: bool) {
    if let Some(window) = web_sys::window() {
        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("toast-notification", &{
            let init = web_sys::CustomEventInit::new();
            let detail = js_sys::Object::new();
            let _ = js_sys::Reflect::set(
                &detail,
                &JsValue::from_str("message"),
                &JsValue::from_str(message),
            );
            let _ = js_sys::Reflect::set(
                &detail,
                &JsValue::from_str("is_error"),
                &JsValue::from_bool(is_error),
            );
            init.set_detail(&detail);
            init.set_bubbles(true);
            init
        }) {
            let _ = window.dispatch_event(&event);
        }
    }
}

fn notify<T>(result: Result<T, String>, success_msg: Option<&str>) -> Result<T, String> {
    match &result {
        Ok(_) => {
            if let Some(msg) = success_msg {
                log_trace(format!("NOTIFY OK : {}", msg));
                emit_toast(msg, false);
            }
        }
        Err(e) => {
            log_trace(format!("NOTIFY ERR: {}", e));
            emit_toast(e, true);
        }
    }
    result
}

pub async fn perform_sync() -> Result<String, String> {
    let res = match safe_invoke("perform_sync", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Sincronización falló: respuesta inesperada".to_string()),
        },
        Err(e) => Err(e),
    };
    if let Ok(ref s) = res {
        emit_toast(s, false);
    }
    if let Err(ref e) = res {
        emit_toast(e, true);
    }
    res
}

pub async fn pull_from_server() -> Result<String, String> {
    let res = match safe_invoke("pull_from_server", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Pull falló: respuesta inesperada".to_string()),
        },
        Err(e) => Err(e),
    };
    if let Ok(ref s) = res {
        emit_toast(s, false);
    }
    if let Err(ref e) = res {
        emit_toast(e, true);
    }
    res
}

pub async fn push_to_server() -> Result<String, String> {
    let res = match safe_invoke("push_to_server", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Push falló: respuesta inesperada".to_string()),
        },
        Err(e) => Err(e),
    };
    if let Ok(ref s) = res {
        emit_toast(s, false);
    }
    if let Err(ref e) = res {
        emit_toast(e, true);
    }
    res
}

pub async fn delete_plan(plan_id: &str) -> Result<(), String> {
    let args = IdArgs {
        id: plan_id.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("delete_plan", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Plan eliminado"))
}

pub async fn generate_week() -> Result<String, String> {
    let res = match safe_invoke("generate_week", JsValue::NULL).await {
        Ok(response) => match response.as_string() {
            Some(result) => Ok(result),
            None => Err("Failed to generate week: response not a string".to_string()),
        },
        Err(e) => Err(e),
    };
    notify(res, Some("Semana generada con éxito"))
}

pub async fn get_index() -> Result<Vec<PlanIndex>, String> {
    let res = match safe_invoke("get_index", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(index) => Ok(index),
            Err(e) => Err(format!("Failed to parse index: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn get_plan_content(id: &str) -> Result<String, String> {
    let args = IdArgs { id: id.to_string() };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("get_plan_content", args_js).await {
        Ok(response) => match response.as_string() {
            Some(content) => Ok(content),
            None => Err("Failed to get plan content: response not a string".to_string()),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn get_plan_detail(id: &str) -> Result<PlanDetail, String> {
    let args = IdArgs { id: id.to_string() };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("get_plan_detail", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn get_config() -> Result<AppConfig, String> {
    let res = match safe_invoke("get_config", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(config) => Ok(config),
            Err(e) => Err(format!("Failed to parse config: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn is_mobile() -> bool {
    match safe_invoke("is_mobile", JsValue::NULL).await {
        Ok(response) => response.as_bool().unwrap_or(false),
        Err(_) => false,
    }
}

pub async fn assign_weekly_plan_to_date(start_date: &str, plan_id: &str) -> Result<(), String> {
    let args = AssignWeeklyPlanArgs {
        start_date: start_date.to_string(),
        plan_id: plan_id.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("assign_weekly_plan_to_date", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Plan semanal asignado con éxito"))
}

pub async fn save_config(config: AppConfig) -> Result<(), String> {
    let args = SaveConfigArgs { config };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("save_config", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Configuración guardada"))
}

pub async fn list_ollama_models() -> Result<Vec<OllamaModel>, String> {
    let res = match safe_invoke("list_ollama_models", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(models) => Ok(models),
            Err(e) => Err(format!("Failed to parse models: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn get_excluded_ingredients() -> Result<Vec<String>, String> {
    let res = match safe_invoke("get_excluded_ingredients", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(ingredients) => Ok(ingredients),
            Err(e) => Err(format!("Failed to parse excluded ingredients: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn get_ingredient_stats() -> Result<Vec<IngredientStats>, String> {
    let res = match safe_invoke("get_ingredient_stats", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(stats) => Ok(stats),
            Err(e) => Err(format!("Failed to parse stats: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn toggle_ingredient_exclusion(ingredient: String) -> Result<Vec<String>, String> {
    let args = ToggleExclusionArgs { ingredient };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("toggle_ingredient_exclusion", args_js).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(ingredients) => Ok(ingredients),
            Err(e) => Err(format!("Failed to parse ingredients: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, Some("Exclusión actualizada"))
}

pub async fn save_excluded_ingredients(ingredients: Vec<String>) -> Result<(), String> {
    let args = SaveExcludedArgs { ingredients };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("save_excluded_ingredients", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Ingredientes guardados"))
}

pub async fn toggle_favorite(plan_id: &str) -> Result<bool, String> {
    let args = PlanIdArgs {
        plan_id: plan_id.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("toggle_favorite", args_js).await {
        Ok(response) => match response.as_bool() {
            Some(fav) => Ok(fav),
            None => Err("Failed to toggle favorite: response not a bool".to_string()),
        },
        Err(e) => Err(e),
    };
    notify(res, Some("Favoritos actualizado"))
}

pub async fn get_plan_metadata(plan_id: &str) -> Result<PlanMetadata, String> {
    let args = PlanIdArgs {
        plan_id: plan_id.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("get_plan_metadata", args_js).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(metadata) => Ok(metadata),
            Err(e) => Err(format!("Failed to parse metadata: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn get_favorite_plans() -> Result<Vec<PlanIndex>, String> {
    let res = match safe_invoke("get_favorite_plans", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(plans) => Ok(plans),
            Err(e) => Err(format!("Failed to parse favorite plans: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn set_plan_rating(plan_id: &str, rating: u8) -> Result<(), String> {
    let args = SetRatingArgs {
        plan_id: plan_id.to_string(),
        rating,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("set_plan_rating", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Calificación guardada"))
}

pub async fn set_plan_note(plan_id: &str, note: String) -> Result<(), String> {
    let args = SetNoteArgs {
        plan_id: plan_id.to_string(),
        note,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("set_plan_note", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Nota guardada"))
}

pub async fn set_plan_display_name(plan_id: &str, display_name: String) -> Result<(), String> {
    let args = SetDisplayNameArgs {
        plan_id: plan_id.to_string(),
        display_name,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("set_plan_display_name", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Nombre del plan actualizado"))
}

pub async fn generate_shopping_list(plan_id: &str) -> Result<ShoppingList, String> {
    let args = PlanIdArgs {
        plan_id: plan_id.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("generate_shopping_list", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, Some("Lista de compras generada"))
}

pub async fn get_shopping_list(plan_id: &str) -> Result<Option<ShoppingList>, String> {
    let args = PlanIdArgs {
        plan_id: plan_id.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("get_shopping_list", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn toggle_shopping_item(
    plan_id: &str,
    item_name: &str,
    checked: bool,
) -> Result<(), String> {
    let args = ToggleItemArgs {
        plan_id: plan_id.to_string(),
        item_name: item_name.to_string(),
        checked,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("toggle_shopping_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Item actualizado"))
}

pub async fn assign_plan_to_date(
    date: String,
    meal_type: MealType,
    plan_id: String,
    recipe_id: Option<String>,
    plan_day_index: Option<u8>,
    assignment_id: Option<String>,
) -> Result<(), String> {
    let args = AssignPlanArgs {
        date,
        meal_type,
        plan_id,
        recipe_id,
        plan_day_index,
        assignment_id,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("assign_plan_to_date", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Plan asignado"))
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
    let invoke_res = safe_invoke("get_calendar_range", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn remove_calendar_entry(date: String, meal_type: MealType) -> Result<(), String> {
    let args = RemoveEntryArgs { date, meal_type };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("remove_calendar_entry", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Entrada eliminada"))
}

pub async fn swap_calendar_entries(
    first_date: String,
    first_meal_type: MealType,
    second_date: String,
    second_meal_type: MealType,
) -> Result<(), String> {
    let args = SwapEntriesArgs {
        first_date,
        first_meal_type,
        second_date,
        second_meal_type,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("swap_calendar_entries", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Recetas intercambiadas"))
}

pub async fn get_statistics() -> Result<Statistics, String> {
    let res = match safe_invoke("get_statistics", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(stats) => Ok(stats),
            Err(e) => Err(format!("Failed to parse statistics: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn get_ingredient_trends() -> Result<Vec<IngredientTrend>, String> {
    let res = match safe_invoke("get_ingredient_trends", JsValue::NULL).await {
        Ok(response) => match serde_wasm_bindgen::from_value(response) {
            Ok(trends) => Ok(trends),
            Err(e) => Err(format!("Failed to parse trends: {}", e)),
        },
        Err(e) => Err(e),
    };
    notify(res, None)
}

pub async fn calculate_nutrition(plan_id: &str) -> Result<PlanNutrition, String> {
    let args = PlanIdArgs {
        plan_id: plan_id.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("calculate_nutrition", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn generate_variation(plan_id: &str, variation: VariationType) -> Result<String, String> {
    let args = VariationArgs {
        plan_id: plan_id.to_string(),
        variation,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("generate_variation", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, Some("Variación generada"))
}

pub async fn preview_recipe_edit(
    plan_id: &str,
    recipe_id: &str,
    prompt: String,
) -> Result<RecipeSuggestion, String> {
    let args = RecipeSuggestionArgs {
        plan_id: plan_id.to_string(),
        recipe_id: recipe_id.to_string(),
        prompt,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("preview_recipe_edit", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn apply_recipe_edit(
    plan_id: &str,
    recipe_id: &str,
    recipe: StructuredRecipe,
) -> Result<PlanDetail, String> {
    let args = ApplyRecipeEditArgs {
        plan_id: plan_id.to_string(),
        recipe_id: recipe_id.to_string(),
        recipe,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("apply_recipe_edit", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, Some("Receta actualizada"))
}

pub async fn search_plans(filters: SearchFilters) -> Result<Vec<PlanIndex>, String> {
    let args = SearchArgs { filters };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("search_plans", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn get_all_tags() -> Result<Vec<Tag>, String> {
    let invoke_res = safe_invoke("get_all_tags", JsValue::NULL).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn create_tag(name: String, color: String) -> Result<Tag, String> {
    let args = CreateTagArgs { name, color };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let invoke_res = safe_invoke("create_tag", args_js).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, Some("Etiqueta creada"))
}

pub async fn delete_tag(id: String) -> Result<(), String> {
    let args = IdArgs { id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("delete_tag", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Etiqueta eliminada"))
}

pub async fn add_tag_to_plan(plan_id: String, tag_id: String) -> Result<(), String> {
    let args = PlanTagArgs { plan_id, tag_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("add_tag_to_plan", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Etiqueta agregada"))
}

pub async fn remove_tag_from_plan(plan_id: String, tag_id: String) -> Result<(), String> {
    let args = PlanTagArgs { plan_id, tag_id };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("remove_tag_from_plan", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Etiqueta desplazada"))
}

pub async fn get_pantry_items() -> Result<Vec<PantryItem>, String> {
    let invoke_res = safe_invoke("get_pantry_items", JsValue::NULL).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn add_pantry_item(item: PantryItem) -> Result<(), String> {
    let args = PantryItemArgs { item };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("add_pantry_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Item agregado"))
}

pub async fn update_pantry_item(item: PantryItem) -> Result<(), String> {
    let args = PantryItemArgs { item };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("update_pantry_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Item actualizado"))
}

pub async fn delete_pantry_item(id: String) -> Result<(), String> {
    let args = IdArgs { id: id.to_string() };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("delete_pantry_item", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Item eliminado"))
}

pub async fn export_data() -> Result<AppBackup, String> {
    let res = match safe_invoke("export_data", JsValue::NULL).await {
        Ok(res) => serde_wasm_bindgen::from_value(res).map_err(|e| e.to_string()),
        Err(e) => Err(e),
    };
    notify(res, Some("Datos preparados para exportar"))
}

pub async fn import_data(backup: AppBackup) -> Result<(), String> {
    let args = ImportDataArgs { backup };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("import_data", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Datos importados exitosamente"))
}

pub async fn get_ui_preferences() -> Result<UIPreferences, String> {
    let invoke_res = safe_invoke("get_ui_preferences", JsValue::NULL).await;
    let res = serde_wasm_bindgen::from_value(invoke_res.map_err(|e| e)?).map_err(|e| e.to_string());
    notify(res, None)
}

pub async fn save_ui_preferences(preferences: UIPreferences) -> Result<(), String> {
    let args = SavePreferencesArgs { preferences };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("save_ui_preferences", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Preferencias guardadas"))
}

pub async fn get_achievements() -> Result<Vec<Achievement>, String> {
    let invoke_res = match safe_invoke("get_achievements", JsValue::NULL).await {
        Ok(response) => serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string()),
        Err(e) => Err(e),
    };
    notify(invoke_res, None)
}

pub async fn send_plan_email(plan_id: String, target_email: String) -> Result<(), String> {
    let args = SendEmailArgs {
        plan_id: plan_id.to_string(),
        target_email: target_email.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let res = match safe_invoke("send_plan_email", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
    notify(res, Some("Email enviado"))
}

pub async fn get_water_intake(date: String) -> Result<WaterRecord, String> {
    let args = GetWaterArgs {
        date: date.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = safe_invoke("get_water_intake", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| e)?).map_err(|e| e.to_string())
}

pub async fn update_water_intake(date: String, current: f32, target: f32) -> Result<(), String> {
    let args = UpdateWaterArgs {
        date: date.to_string(),
        current,
        target,
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match safe_invoke("update_water_intake", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn get_water_history(
    start_date: String,
    end_date: String,
) -> Result<std::collections::HashMap<String, WaterRecord>, String> {
    let args = GetWaterHistoryArgs {
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
    };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let result = safe_invoke("get_water_history", args_js).await;
    serde_wasm_bindgen::from_value(result.map_err(|e| e)?).map_err(|e| e.to_string())
}

pub async fn get_sync_status() -> Result<SyncStatus, String> {
    let res = safe_invoke("get_sync_status", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(res.map_err(|e| e)?).map_err(|e| e.to_string())
}
