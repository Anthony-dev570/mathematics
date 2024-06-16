use crate::shared::traits::number::Number;

pub trait Shape<N: Number> {
    fn area(&self) -> N;
}