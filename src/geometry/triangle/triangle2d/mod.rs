use crate::linear_algebra::vector::types::Vector2;
use crate::shared::traits::number::Number;

pub mod imp;

pub enum Triangle2D<N: Number> {
    FromVertices {
        a: Vector2<N>,
        b: Vector2<N>,
        c: Vector2<N>
    }
}