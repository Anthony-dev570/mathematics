use crate::geometry::shape::Shape;
use crate::shared::traits::number::Number;

pub mod triangle2d;

pub trait Triangle<N: Number>: Shape<N> {
    fn base(&self) -> N;
    fn height(&self) -> N;
}