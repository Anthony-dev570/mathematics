pub mod imp;

#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    celsius: f64,
    kelvin: f64,
    fahrenheit: f64
}