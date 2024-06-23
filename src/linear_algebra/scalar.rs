use std::ops::Neg;
use crate::shared::traits::number::Number;

pub trait Scalar: Number + Neg<Output=Self> {
    const NEG_ONE: Self;
}

impl Scalar for f32 {
    const NEG_ONE: Self = -1_f32;
}
impl Scalar for f64 {
    const NEG_ONE: Self = -1_f64;
}