mod plot_trend;
pub mod single_chart;
pub mod trend_chart;

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[derive(Debug, Clone)]
pub struct PlotConfig {
    pub width: u32,
    pub height: u32,
    pub element_id: String,
    pub is_dark: bool,
}

#[derive(Debug, Clone)]
pub enum PlotType {
    Latency,
    Throughput,
}

#[wasm_bindgen]
extern "C" {
    type EChartsInstance;

    #[wasm_bindgen(js_namespace = echarts)]
    fn getInstanceByDom(element: &Element) -> Option<EChartsInstance>;

    #[wasm_bindgen(method)]
    fn dispose(this: &EChartsInstance);
}

pub fn dispose_chart(element_id: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(element_id) {
                if let Some(instance) = getInstanceByDom(&element) {
                    instance.dispose();
                }
            }
        }
    }
}
