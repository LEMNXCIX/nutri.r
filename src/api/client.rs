use crate::types::*;
use wasm_bindgen::prelude::*;

pub async fn call_api_fallback(cmd: &str, args: JsValue, api_base: &str) -> Result<JsValue, String> {
    let client = reqwest::Client::new();

    match cmd {
        "get_index" => {
            let res = client
                .get(format!("{}/plans", api_base))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let index = res.json::<Vec<PlanIndex>>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&index).map_err(|e| e.to_string())
        }
        "get_plan_content" => {
            let plan_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/plans/{}/content", api_base, plan_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let content = res.text().await.map_err(|e| e.to_string())?;
            Ok(JsValue::from_str(&content))
        }
        "get_plan_detail" => {
            let plan_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            let res = client
                .get(format!("{}/plans/{}", api_base, plan_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let detail = res.json::<PlanDetail>().await.map_err(|e| e.to_string())?;
            serde_wasm_bindgen::to_value(&detail).map_err(|e| e.to_string())
        }
        "save_plan" => {
            let detail: PlanDetail =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/plans", api_base))
                .json(&detail)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
        "delete_plan" => {
            let plan_args: IdArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .delete(format!("{}/plans/{}", api_base, plan_args.id))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
        }
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
            let config_args: SaveConfigArgs =
                serde_wasm_bindgen::from_value(args).map_err(|e| e.to_string())?;
            client
                .post(format!("{}/config", api_base))
                .json(&config_args.config)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(JsValue::NULL)
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
        // ... (resto de comandos serán añadidos progresivamente)
        _ => {
            let err_msg = format!("Command '{}' not implemented for web fallback", cmd);
            Err(err_msg)
        }
    }
}

pub async fn check_endpoint(url: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(res) if res.status().is_success() => Ok(()),
        Ok(res) => Err(format!("Error: {}", res.status())),
        Err(e) => Err(e.to_string()),
    }
}
