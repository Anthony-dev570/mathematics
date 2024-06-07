use crate::matrix::types::SquareMatrix;
use crate::number::Number;

impl <const L: usize, N: Number> SquareMatrix<L, N> {
    pub fn identity() -> Self {
        let mut o = Self::default();
        for i in 0..L {
            o.0[i][i] = N::ONE;
        }
        o
    }
}