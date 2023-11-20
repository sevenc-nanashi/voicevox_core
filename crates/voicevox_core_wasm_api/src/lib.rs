use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", voicevox_core::VERSION));
}
#[wasm_bindgen(start)]
pub fn start() {
    alert(&format!("Hello, {}!", voicevox_core::VERSION));
}
