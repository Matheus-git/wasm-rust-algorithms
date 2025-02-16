pub fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * (9.0 / 5.0)) + 32.0
}

pub fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}