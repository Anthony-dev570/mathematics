use std::fmt::{Display, Formatter};
use crate::matrix::Matrix;
use crate::number::Number;
use crate::traits::map_iter::{MapIter, MapSegment};

impl<const C: usize, const R: usize, N: Number> Matrix<C, R, N> {
    pub fn new(matrix: [[N; C]; R]) -> Self {
        Self(matrix)
    }

    pub fn raw(self) -> [[N; C]; R] {
        self.0
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
            let s = n.to_string();
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