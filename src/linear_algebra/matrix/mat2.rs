use crate::linear_algebra::matrix::types::Mat2;
use crate::shared::traits::number::Number;

impl <N: Number> Mat2<N> {
    crate::to_mat! {
        N, 2 => 3, 4, 5, 6, 7, 8, 9
    }
}