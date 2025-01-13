use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn location() -> JsValue;
}

pub fn get_api_base_url() -> String {
    let window = match web_sys::window() {
        Some(win) => win,
        None => return "http://127.0.0.1:8081".to_string(),
    };

    // Try to get from window.API_BASE_URL if it exists (can be set in index.html)
    if let Some(api_url) = Reflect::get(&window, &JsValue::from_str("API_BASE_URL"))
        .ok()
        .and_then(|val| val.as_string())
    {
        return api_url;
    }

    // If not found, construct from current location
    if let Ok(location) = window.location().host() {
        // If we're on a custom port, assume the API is on port 8081
        if let Some(colon_pos) = location.find(':') {
            return format!(
                "http://{}",
                location.replace(&location[colon_pos..], ":8081")
            );
        }
        // Otherwise use the same host with port 8081
        return format!("http://{}:8081", location);
    }

    // Fallback to localhost
    "http://127.0.0.1:8081".to_string()
}
