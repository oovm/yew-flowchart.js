use flowchartjs_wasmbind::draw_flow_chart;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[test]
fn run() {
    println!("it works!")
}

#[wasm_bindgen_test]
fn mode() {
    assert_ne!(d.render("\\frac12"), i.render("\\frac12"));
}
