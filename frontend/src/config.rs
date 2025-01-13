use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn location() -> JsValue;
}

pub fn get_api_base_url() -> String {
    if cfg!(debug_assertions) {
        // In debug mode, always use localhost
        return "http://127.0.0.1:8081".to_string();
    }

    // In release mode, try to get from window.API_BASE_URL if it exists
    let window = match web_sys::window() {
        Some(win) => win,
        None => return "https://benchmarks.iggy.rs".to_string(),
    };

    if let Some(api_url) = Reflect::get(&window, &JsValue::from_str("API_BASE_URL"))
        .ok()
        .and_then(|val| val.as_string())
    {
        return api_url;
    }

    // For other hosts in release mode, use the same host with port 8081
    if let Ok(location) = window.location().host() {
        if let Some(colon_pos) = location.find(':') {
            return format!("https://{}", location.replace(&location[colon_pos..], ""));
        }
        return format!("https://{}", location);
    }

    // Fallback to production URL
    "https://benchmarks.iggy.rs".to_string()
}
