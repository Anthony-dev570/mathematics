use std::ops::{Div, Index, IndexMut, Mul, Neg};
use crate::linear_algebra::vector::Vector;
use crate::shared::traits::number::Number;

impl <const L: usize, N: Number> Mul<N> for Vector<L, N> {
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] *= rhs;
        }
        out
    }
}

impl <const L: usize, N: Number> Div<N> for Vector<L, N> {
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] /= rhs;
        }
        out
    }
}

impl <const L: usize, N: Number + Neg<Output=N>> Neg for Vector<L, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = self.clone();

        for i in 0..L {
            out[i] = -out[i];
        }

        out
    }
}

impl <const L: usize, N: Number> Index<usize> for Vector<L, N> {
    type Output = N;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <const L: usize, N: Number> IndexMut<usize> for Vector<L, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
