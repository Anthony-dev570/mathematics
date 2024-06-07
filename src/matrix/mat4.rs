use crate::matrix::types::Mat4;
use crate::number::Number;

impl <N: Number> Mat4<N> {
    crate::to_mat!{
        N, 4 => 2, 3, 5, 6, 7, 8, 9
    }
}