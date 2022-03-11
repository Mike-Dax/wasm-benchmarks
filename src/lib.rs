use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn find_minimum(values: &[f64]) -> f64 {
    let mut min = f64::INFINITY;

    for &num in values {
        if num < min {
            min = num
        }
    }

    min
}
