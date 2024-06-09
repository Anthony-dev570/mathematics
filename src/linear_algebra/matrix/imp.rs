use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use crate::linear_algebra::matrix::Matrix;
use crate::shared::traits::map_iter::{MapIter, MapSegment};
use crate::shared::traits::number::Number;

impl<const C: usize, const R: usize, N: Number> Matrix<C, R, N> {
    pub fn new(matrix: [[N; C]; R]) -> Self {
        Self(matrix)
    }

    pub fn raw(self) -> [[N; C]; R] {
        self.0
    }

    pub fn transpose(self) -> Matrix<R, C, N> {
        let mut out = Matrix::new([[N::ZERO; R]; C]);

        for i in 0..R {
            for j in 0..C {
                out[j][i] = self[i][j];
            }
        }

        out
    }
}


impl<const C: usize, const R: usize, N: Number> Default for Matrix<C, R, N> {
    fn default() -> Self {
        Self::new([[N::default(); C]; R])
    }
}

impl<const C: usize, const R: usize, N: Number> Display for Matrix<C, R, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut length = 0;
        let m = self.0.iter().map(|i| i.iter().map(|n| {
            let s = format!("{:?}", n);
            if length < s.len() {
                length = s.len();
            }
            s
        }).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

        let t = m.map_iter(|(a, b)| {
            let o = a.map_iter(|(ab, bb)| {
                let mut s = ab.to_string();
                while s.len() != length {
                    s = format!(" {}", s);
                }
                s
            }).join(", ");
            match b {
                MapSegment::Beginning => format!("⎡ {} ⎤", o),
                MapSegment::Segment => format!("⎢ {} ⎥", o),
                MapSegment::End => format!("⎣ {} ⎦", o)
            }
        }).join("\n");
        f.write_str(&t)
    }
}

impl<const C: usize, const R: usize, N: Number> Index<usize> for Matrix<C, R, N> {
    type Output = [N; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const C: usize, const R: usize, N: Number> IndexMut<usize> for Matrix<C, R, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}