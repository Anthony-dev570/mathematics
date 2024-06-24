use crate::linear_algebra::vector::Vector;
use crate::shared::traits::number::Number;

pub mod rect;

pub trait Shape<const L: usize, N: Number> {
    fn area(&self) -> N;

    fn contains_point(&self, point: &Vector<L, N>) -> bool;
}