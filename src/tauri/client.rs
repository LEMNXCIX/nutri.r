use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub fn is_tauri() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().expect("no global `window` exists");
        let tauri = js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI__"));
        tauri.is_ok() && !tauri.unwrap().is_undefined()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}
