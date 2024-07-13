use std::cmp::Ordering;
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

    pub fn pivots(&self) -> [Option<(N, usize)>; R] {
        let mut pivots = [None; R];

        for r in 0..R {
            for c in 0..C {
                let v = self[r][c];
                if v != N::ZERO {
                    pivots[r] = Some((v, c));
                    break;
                }
            }
        }

        pivots
    }

    pub fn is_row_echelon_form(&self) -> bool {
        let pivots = self.pivots();

        let mut last_pivot = pivots[0];
        for i in 1..pivots.len() {
            let pivot = pivots[i];

            if last_pivot.is_none() && pivot.is_some() {
                return false;
            }
            if let Some((_lp_n, lpi)) = last_pivot {
                if let Some((_p_n, pi)) = pivot {
                    if pi <= lpi {
                        return false;
                    }
                }
            }

            last_pivot = pivot;
        }
        true
    }

    pub fn is_reduced_row_echelon_form(&self) -> bool {
        let pivots = self.pivots();

        let mut last_pivot = pivots[0];
        for i in 1..pivots.len() {
            let pivot = pivots[i];

            if last_pivot.is_none() && pivot.is_some() {
                return false;
            }
            if let Some((_lp_n, lpi)) = last_pivot {
                if let Some((_p_n, pi)) = pivot {
                    if pi <= lpi || _lp_n != N::ONE {
                        return false;
                    }
                }
            }

            last_pivot = pivot;
        }
        true
    }

    pub fn to_reduced_row_echelon_form(&self) -> Self {
        let mut echelon = self.to_row_echelon_form();

        let pivots = echelon.pivots();

        for row in 0..R {
            let pivot = pivots[row];
            match pivot {
                None => break,
                Some((pivot_value, _)) => {
                    let pivot_value = N::ONE / pivot_value;
                    echelon[row].iter_mut().for_each(|s| *s *= pivot_value);
                }
            }
        }

        echelon
    }

    pub fn to_row_echelon_form(&self) -> Self {
        let mut out: Self = self.clone();
        let mut pivots = out.pivots();

        let mut v = out.0.into_iter().enumerate().collect::<Vec<(usize, [N; C])>>();

        v.sort_by(|(a, _), (b, _)| {
            let mut order = Ordering::Equal;
            let a_pivot = pivots[*a];
            let b_pivot = pivots[*b];
            if b_pivot.is_none() {
                order = Ordering::Less;
            } else if a_pivot.is_none() {
                order = Ordering::Greater;
            } else {
                let a_pivot = a_pivot.unwrap().1;
                let b_pivot = b_pivot.unwrap().1;
                order = a_pivot.cmp(&b_pivot);
            }

            order
        });

        for row in 0..R {
            out[row] = v[row].1;
        }

        out
    }

    pub fn remove_row(&self, rem: usize) -> Matrix<C, {R - 1}, N> {
        let mut out = Matrix::default();

        let mut j = 0;
        for i in 0..R {
            if i == rem {
                continue;
            }
            out[j] = self[i];
            j += 1;
        }

        out
    }

    pub fn add_row(&self) -> Matrix<C, {R + 1}, N> {
        let mut out = Matrix::default();
        for r in 0..R {
            out[r] = self[r];
        }
        out
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        let _a = self[a].clone();
        self[a] = self[b];
        self[b] = _a;
    }

    pub fn mul_row(&mut self, row: usize, scalar: N) {
        for c in &mut self[row] {
            *c *= scalar;
        }
    }

    pub fn mul_col(&mut self, col: usize, scalar: N) {
        for i in 0..R {
            self[i][col] *= scalar;
        }
    }

    pub fn mul_add_row(&mut self, i: usize, j: usize, scalar: N) {
        let mut row = self[j].clone();
        for r in &mut row {
            *r *= scalar;
        }
        for t in 0..C {
            self[i][t] += row[t];
        }
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