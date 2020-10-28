use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen(module = "/src/flowchart.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = draw_flow_chart)]
    pub fn draw_flow_chart(e: &Element, expr: &str);
}
