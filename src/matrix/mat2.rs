use crate::matrix::types::Mat2;
use crate::number::Number;

impl <N: Number> Mat2<N> {
    crate::to_mat! {
        N, 2 => 3, 4, 5, 6, 7, 8, 9
    }
}