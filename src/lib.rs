use wasm_bindgen::prelude::*;

pub mod temperature;
pub mod quicksort;
pub mod binary_search;

use crate::temperature::{convert_fahrenheit_to_celsius, convert_celsius_to_fahrenheit};
use crate::quicksort::calc_quicksort;
use crate::binary_search::binary_search as calc_binary_search;

#[wasm_bindgen]
pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    convert_celsius_to_fahrenheit(celsius)
}

#[wasm_bindgen]
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    convert_fahrenheit_to_celsius(fahrenheit)
}

#[wasm_bindgen]
pub fn quicksort(list: Vec<i32>) -> Vec<i32> {
    calc_quicksort(list)
}

#[wasm_bindgen]
pub fn binary_search(x: i32, max: i32) -> Option<usize> {
    calc_binary_search(x, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_convert_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[wasm_bindgen_test]
    fn test_convert_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
    }
}
