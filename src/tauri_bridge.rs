use crate::api;
use crate::tauri::{self, invoke, is_tauri};
pub use crate::types::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

static DEBUG_LOGS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
static IS_API_ONLINE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn log_trace(msg: String) {
    #[cfg(target_arch = "wasm32")]
    log(&msg);

    if let Ok(mut logs) = DEBUG_LOGS.lock() {
        logs.push(msg.clone());
        if logs.len() > 100 {
            logs.remove(0);
        }
    }

    if let Some(window) = web_sys::window() {
        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("debug-log", &{
            let init = web_sys::CustomEventInit::new();
            init.set_detail(&JsValue::from_str(&msg));
            init
        }) {
            let _ = window.dispatch_event(&event);
        }
    }
}

pub fn get_debug_logs() -> Vec<String> {
    if let Ok(logs) = DEBUG_LOGS.lock() {
        logs.clone()
    } else {
        Vec::new()
    }
}

pub fn clear_debug_logs() {
    if let Ok(mut logs) = DEBUG_LOGS.lock() {
        logs.clear();
    }
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
    if use_tauri && !is_config_cmd && !is_sync_cmd {
        match invoke("is_mobile", JsValue::NULL).await {
            Ok(js_val) => {
                if js_val.as_bool().unwrap_or(false) {
                    use_tauri = false;
                }
            }
            Err(_) => {}
        }
    }

    // Unified Write Strategy for Tauri environments (Desktop/Mobile)
    // We ALWAYS write to the local store first to ensure offline data is up-to-date.
    if is_tauri() && is_write && !is_config_cmd {
        log_trace(format!("WRITE_LOCAL_FIRST: Persisting {} to local store", cmd));
        let _ = invoke(cmd, args.clone()).await;
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

        let is_online = if let Ok(health) = IS_API_ONLINE.lock() {
            *health
        } else {
            true
        };

        if !is_config_cmd && !is_online {
            log_trace(format!(
                "FALLBACK: API marked as OFFLINE, skipping health check for {}",
                cmd
            ));
            return invoke(cmd, args).await.map_err(|e| format!("{:?}", e));
        }

        log_trace(format!("REQ: {} a {}", cmd, api_base));
        let res = api::call_api_fallback(cmd, args.clone(), &api_base).await;

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
    day_id: &str,
    prompt: String,
) -> Result<RecipeSuggestion, String> {
    let args = RecipeSuggestionArgs {
        plan_id: plan_id.to_string(),
        day_id: day_id.to_string(),
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

pub async fn check_endpoint(url: &str) -> Result<(), String> {
    api::check_endpoint(url).await
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
    "http://localhost:3001/api".to_string()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapEntriesArgs {
    pub first_date: String,
    pub first_meal_type: MealType,
    pub second_date: String,
    pub second_meal_type: MealType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyRecipeEditArgs {
    pub plan_id: String,
    pub recipe_id: String,
    pub recipe: StructuredRecipe,
}
