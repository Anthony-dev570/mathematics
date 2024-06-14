use std::fmt::Display;
use std::ops::{Div, Index, IndexMut, Neg};
use crate::linear_algebra::vector::Vector;
use crate::shared::traits::number::Number;
use crate::shared::traits::lerp::Lerp;

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

impl <const L: usize, N: Number> Display for Vector<L, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("<{}>", self.0.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ")))
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

impl <const L: usize, N: Number> Vector<L, N> {
    ///Create a new N dimensional 1xN matrix(vector) with provided values.
    pub fn new(values: [N; L]) -> Self {
        Self(values)
    }

    ///Deconstructs this vector and returns its inner 1xN data.
    pub fn take(self) -> [N; L] {
        self.0
    }


    ///Calculates the magnitude(length) of this vector. Calculated by raising the vector's components to the second power, summing them then getting the square root.
    ///<br>Example: √(x^2 + y^2 + ...)
    ///<br>See <a href='https://en.wikipedia.org/wiki/Magnitude_(mathematics)'>Magnitude</a>
    pub fn magnitude(&self) -> N {
        self.0.iter().map(|n| n.num_pow(N::TWO)).sum::<N>().num_sqrt()
    }

    ///Sums the components of the vector.
    pub fn sum(&self) -> N {
        self.0.iter().map(|s| *s).sum()
    }

    ///Divide all components of this vector by this vector's magnitude, resulting in a vector with a magnitude of 1 (a unit vector).
    ///<br>v` = v / |v|
    ///<br>See <a href="https://en.wikipedia.org/wiki/Unit_vector">Unit Vector</a>
    pub fn normalize(mut self) -> Self {
        let magnitude = self.magnitude();
        for f in &mut self.0 {
            *f /= magnitude;
        }
        self.clone()
    }

    ///Performs vector normalization on a copy of this vector.
    pub fn normalized(&self) -> Self {
        self.clone().normalize()
    }

    ///Computes the Euclidean distance between this vector, and another.
    ///<br>This is done by taking the difference of the components, squaring them, summing them then getting the square root of the sum.
    ///<br>Ex: <a, b, c> -> <d, e, f> => √((d-a)^2 + (e-b)^2 + (f-c)^2)
    ///<br>See: <a href="https://en.wikipedia.org/wiki/Euclidean_distance">Euclidean distance</a>
    pub fn distance(&self, b: &Self) -> N {
        (0..L).into_iter().map(|index|
            (b[index] - self[index]).num_pow(N::TWO)
        ).sum::<N>().num_sqrt()
    }

    ///Computes the mathematical dot product of two vectors, calculated by summing the products of each component of the vectors.
    ///<br>Ex: <a, b, c> * <d, e, f> = ad + be + cf
    ///<br>See: <a href="https://en.wikipedia.org/wiki/Dot_product">Dot Product</a>
    pub fn dot(&self, b: &Self) -> N {
        (0..L).into_iter().map(|index| self[index] * b[index]).sum::<N>()
    }

    ///Creates a new vector with the max of the individual components of two vectors.
    ///<br>Ex: <1, 4, 6> maxed with <2, 0, 4> = <2, 4, 6>
    ///<br>See: <a href="https://en.wikipedia.org/wiki/Maximum_and_minimum">Maximum and minimum</a>
    pub fn max(&self, b: &Self) -> Self {
        let mut out = self.0.clone();
        for i in 0..L {
            out[i] = out[i].num_max(&b[i]);
        }
        Self(out)
    }

    ///Creates a new vector with the min of the individual components of two vectors.
    ///<br>Ex: <1, 4, 6> maxed with <2, 0, 4> = <1, 0, 4>
    ///<br>See: <a href="https://en.wikipedia.org/wiki/Maximum_and_minimum">Maximum and minimum</a>
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