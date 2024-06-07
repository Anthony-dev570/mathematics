use crate::matrix::types::Mat7;
use crate::number::Number;

impl <N: Number> Mat7<N> {
    crate::to_mat!{
        N, 7 => 2, 3, 4, 5, 6, 8, 9
    }
}