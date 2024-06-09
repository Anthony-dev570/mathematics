use crate::chemistry::element::classification::Classification;
use crate::chemistry::phase::Phase;
use crate::chemistry::temperature::Temperature;

pub mod imp;
pub mod classification;

#[derive(Debug, Clone, Copy)]
pub struct Element<'a> {
    atomic_number: u8,
    symbol: &'a str,
    name: &'a str,
    group: u8,
    period: u8,
    block: char,
    atomic_weight: f64,
    density: f64,
    melting_point: Option<Temperature>,
    boiling_point: Temperature,
    specific_heat_capacity: f64,
    electro_negativity: Option<f64>,
    phase: Phase,
    classification: Classification
}