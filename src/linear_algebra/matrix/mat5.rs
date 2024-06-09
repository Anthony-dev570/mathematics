use crate::linear_algebra::matrix::types::Mat5;
use crate::shared::traits::number::Number;

impl<N: Number> Mat5<N> {
    crate::to_mat! {
        N, 5 => 2, 3, 4, 6, 7, 8, 9
    }
}