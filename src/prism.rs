#[cfg(target_arch = "wasm32")]
mod prism {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Prism, js_name = highlightAll)]
        fn highlight_all();
    }

    pub fn highlight() {
        // crida segura: si Prism no existeix, peta a JS, però això només passa
        // si no has carregat el script correctament
        highlight_all();
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod prism {
    // no-op al servidor
    pub fn highlight() {}
}

pub use prism::highlight;
