use crate::shared::traits::number::Number;

pub mod imp;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Angle<N: Number> {
    Radians(N),
    Degrees(N)
}