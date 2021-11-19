use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

pub fn rand_usize(min: usize, max: usize) -> usize {
    (min.wrapping_add(max.wrapping_sub(min)) as f64 * random()) as usize
}

pub const fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

pub const fn min(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

pub const fn clamp(x: i32, low: i32, high: i32) -> i32 {
    min(max(low, x), high)
}
