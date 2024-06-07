use crate::matrix::types::Mat9;
use crate::number::Number;

impl <N: Number> Mat9<N> {
    crate::to_mat!{
        N, 9 => 2, 3, 4, 5, 6, 7, 8
    }
}