use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(x: usize, y: usize) -> usize{
    x + y
}
