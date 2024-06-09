use crate::linear_algebra::matrix::types::SquareMatrix;
use crate::shared::traits::number::Number;

impl <const L: usize, N: Number> SquareMatrix<L, N> {
    pub fn identity() -> Self {
        let mut o = Self::default();
        for i in 0..L {
            o.0[i][i] = N::ONE;
        }
        o
    }
}