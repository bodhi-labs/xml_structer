#![cfg(feature = "wasm")]

use crate::validate::run;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct WasmReport {
    json: String,
    html: String,
}

#[wasm_bindgen]
impl WasmReport {
    #[wasm_bindgen(getter)]
    pub fn json(&self) -> String {
        self.json.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn html(&self) -> String {
        self.html.clone()
    }
}

#[wasm_bindgen]
pub fn check_xml(xml: &str) -> WasmReport {
    let rep = run(xml).expect("validation crashed");
    let json = rep.to_json_string().unwrap();

    // super-small HTML renderer
    let mut html = String::from(r#"<div class="report">"#);
    for m in &rep.errors {
        html.push_str(&format!(
            r#"<div class="err">✗ {}:{}  {}</div>"#,
            m.line, m.column, m.text
        ));
    }
    for m in &rep.warnings {
        html.push_str(&format!(
            r#"<div class="warn">⚠ {}:{}  {}</div>"#,
            m.line, m.column, m.text
        ));
    }
    if rep.is_valid() && rep.warnings.is_empty() {
        html.push_str(r#"<div class="ok">✔ Validation passed</div>"#);
    }
    html.push_str("</div>");
    WasmReport { json, html }
}
