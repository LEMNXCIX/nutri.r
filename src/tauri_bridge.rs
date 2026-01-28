use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlanIndex {
    pub id: String,
    pub fecha: String,
    pub proteinas: Vec<String>,
    pub enviado: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub prompt_maestro: String,
    pub smtp_user: String,
    pub smtp_password: String,
}

#[derive(Serialize)]
struct SaveConfigArgs {
    config: AppConfig,
}

#[derive(Serialize)]
struct GetPlanArgs<'a> {
    id: &'a str,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window","__TAURI__","core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
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
    let args = GetPlanArgs { id };
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

pub async fn save_config(config: AppConfig) -> Result<(), String> {
    let args = SaveConfigArgs { config };
    let args_js = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    match invoke("save_config", args_js).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
