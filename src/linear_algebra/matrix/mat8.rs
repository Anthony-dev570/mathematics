use crate::linear_algebra::matrix::types::Mat8;
use crate::shared::traits::number::Number;

impl <N: Number> Mat8<N> {
    crate::to_mat!{
        N, 8 => 2, 3, 4, 5, 6, 7, 9
    }
}