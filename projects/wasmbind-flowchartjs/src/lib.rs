use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/flowchart.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = draw_flow_chart)]
    pub fn draw_flow_chart(expr: &str) -> String;
}
