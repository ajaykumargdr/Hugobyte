use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("{}!", name));
}

// cargo build --target wasm32-unknown-unknown
// wasm-bindgen --target web --out-dir pkg ./target/wasm32-unknown-unknown/debug/wasm_building_1.wasm  

// rustup target add wasm32-wasi

// cargo build --release --target wasm32-wasi 