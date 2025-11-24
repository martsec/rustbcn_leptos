// src/mermaid.rs
#[cfg(target_arch = "wasm32")]
mod inner {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsValue;

    #[wasm_bindgen]
    extern "C" {
        // JS: mermaid.init(config, selector)
        // config pot ser undefined; selector = ".mermaid"
        #[wasm_bindgen(js_namespace = mermaid, js_name = init)]
        fn mermaid_init(config: JsValue, selector: &str);
    }

    pub fn rerender_all() {
        // scan de tots els elements amb class="mermaid"
        mermaid_init(JsValue::UNDEFINED, ".mermaid");
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod inner {
    pub fn rerender_all() {
        // no-op al servidor
    }
}

pub use inner::rerender_all;
