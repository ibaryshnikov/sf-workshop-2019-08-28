use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::console::log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! log_js {
    ($v:ident) => ($crate::console::log_js_value(&$v))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_js_value(v: &JsValue);
}
