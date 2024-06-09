use crate::linear_algebra::matrix::types::Mat6;
use crate::shared::traits::number::Number;

impl <N: Number> Mat6<N> {
    crate::to_mat!{
        N, 6 => 2, 3, 4, 5, 7, 8, 9
    }
}