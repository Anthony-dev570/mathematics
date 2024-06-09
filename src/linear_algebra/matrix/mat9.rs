use crate::linear_algebra::matrix::types::Mat9;
use crate::shared::traits::number::Number;

impl <N: Number> Mat9<N> {
    crate::to_mat!{
        N, 9 => 2, 3, 4, 5, 6, 7, 8
    }
}