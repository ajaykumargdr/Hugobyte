// wasm32-wasi will not work 

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("{}!", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b:i32) -> i32{
    a + b
}