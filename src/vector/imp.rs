use std::fmt::Display;
use std::ops::{Index, IndexMut};
use crate::number::Number;
use crate::traits::lerp::Lerp;
use crate::vector::Vector;

impl <const L: usize, N: Number> Display for Vector<L, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("<{}>", self.0.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ")))
    }
}

impl <const L: usize, N: Number> Vector<L, N> {
    pub fn new(values: [N; L]) -> Self {
        Self(values)
    }

    pub fn take(self) -> [N; L] {
        self.0
    }

    pub fn magnitude(&self) -> N {
        self.0.iter().map(|n| n.num_pow(N::TWO)).sum::<N>().num_sqrt()
    }

    pub fn sum(&self) -> N {
        self.0.iter().map(|s| *s).sum()
    }

    pub fn normalize(mut self) -> Self {
        let magnitude = self.magnitude();
        for f in &mut self.0 {
            *f /= magnitude;
        }
        self.clone()
    }

    pub fn normalized(&self) -> Self {
        self.clone().normalize()
    }

    pub fn distance(&self, b: &Self) -> N {
        (0..L).into_iter().map(|index|
            (b[index] - self[index]).num_pow(N::TWO)
        ).sum::<N>().num_sqrt()
    }

    pub fn dot(&self, b: &Self) -> N {
        (0..L).into_iter().map(|index| self[index] * b[index]).sum::<N>()
    }

    pub fn max(&self, b: &Self) -> Self {
        let mut out = self.0.clone();
        for i in 0..L {
            out[i] = out[i].num_max(&b[i]);
        }
        Self(out)
    }

    pub fn min(&self, b: &Self) -> Self {
        let mut out = self.0.clone();
        for i in 0..L {
            out[i] = out[i].num_min(&b[i]);
        }
        Self(out)
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

impl <const L: usize, N: Number> Default for Vector<L, N> {
    fn default() -> Self {
        Self([N::default(); L])
    }
}

impl <const L: usize, N: Number> Lerp for Vector<L, N> {
    fn lerp(&self, b: &Self, t: f32) -> Self {
        let mut out = self.clone();
        for i in 0..L {
            out.0[i] = out.0[i].lerp(&b.0[i], t);
        }
        out
    }
}