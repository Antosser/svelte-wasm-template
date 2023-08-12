extern crate console_error_panic_hook;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use std::panic;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_wasm() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn add(a: f32, b: f32) -> f32 {
    a + b
}

#[wasm_bindgen]
pub fn pythagorean(a: f32, b: f32) -> f32 {
    (a * a + b * b).sqrt()
}

pub fn set_panic_hook() {
    // Call the `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
