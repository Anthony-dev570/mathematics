use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::linear_algebra::quaternion::{Quaternion, QuaternionConstructor};
use crate::linear_algebra::vector::types::Vector3;
use crate::linear_algebra::vector::Vector;
use crate::shared::traits::number::Number;

impl<N: Number> Mul<Vector3<N>> for Quaternion<N> where N: Neg<Output=N>{
    type Output = Vector3<N>;

    fn mul(self, rhs: Vector3<N>) -> Self::Output {
        let q = Self::new(QuaternionConstructor::Pure {
            xyz: rhs,
        });
        (self * q * self.conjugate()).xyz
    }
}
impl<N: Number> Mul<N> for Quaternion<N> where N: Neg<Output=N>{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self {
            xyz: self.xyz * rhs,
            w: self.w * rhs,
        }
    }
}
impl<N: Number> Mul<Quaternion<N>> for Quaternion<N> {
    type Output = Self;

    fn mul(self, rhs: Quaternion<N>) -> Self::Output {
        let q = self.wxyz();
        let r = rhs.wxyz();
        let t = [
            r[0] * q[0] - r[1] * q[1] - r[2] * q[2] - r[3] * q[3],
            r[0] * q[1] + r[1] * q[0] - r[2] * q[3] + r[3] * q[2],
            r[0] * q[2] + r[1] * q[3] + r[2] * q[0] - r[3] * q[1],
            r[0] * q[3] - r[1] * q[2] + r[2] * q[1] + r[3] * q[0]
        ];
        Self {
            xyz: Vector::new([t[1], t[2], t[3]]),
            w: t[0],
        }
    }
}

impl<N: Number> Div<Quaternion<N>> for Quaternion<N> {
    type Output = Self;

    fn div(self, rhs: Quaternion<N>) -> Self::Output {
        let (q0, q1, q2, q3) = self.components();
        let (r0, r1, r2, r3) = rhs.components();

        let denominator = r0.num_pow(N::TWO) + r1.num_pow(N::TWO) + r2.num_pow(N::TWO) + r3.num_pow(N::TWO);

        Self {
            xyz: Vector3::new([
                (r0 * q1 - r1 * q0 - r2 * q3 + r3 * q2) / denominator,
                (r0 * q2 + r1 * q3 - r2 * q0 - r3 * q1) / denominator,
                (r0 * q3 - r1 * q2 + r2 * q1 - r3 * q0) / denominator
            ]),
            w: (r0 * q0 + r1 * q1 + r2 * q2 + r3 * q3) / denominator,
        }
    }
}
impl<N: Number> Div<N> for Quaternion<N> {
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        let inverse = N::ONE / rhs;

        Self {
            xyz: self.xyz * inverse,
            w: self.w * inverse,
        }
    }
}

impl<N: Number> Add<Quaternion<N>> for Quaternion<N> {
    type Output = Self;

    fn add(self, rhs: Quaternion<N>) -> Self::Output {
        Self {
            xyz: self.xyz + rhs.xyz,
            w: self.w + rhs.w,
        }
    }
}

impl<N: Number> Sub<Quaternion<N>> for Quaternion<N> {
    type Output = Self;

    fn sub(self, rhs: Quaternion<N>) -> Self::Output {
        Self {
            xyz: self.xyz - rhs.xyz,
            w: self.w - rhs.w,
        }
    }
}


impl<N: Number> AddAssign<Quaternion<N>> for Quaternion<N> {
    fn add_assign(&mut self, rhs: Quaternion<N>) {
        *self = *self + rhs;
    }
}

impl<N: Number> SubAssign<Quaternion<N>> for Quaternion<N> {
    fn sub_assign(&mut self, rhs: Quaternion<N>) {
        *self = *self - rhs;
    }
}

impl<N: Number> MulAssign<Quaternion<N>> for Quaternion<N> {
    fn mul_assign(&mut self, rhs: Quaternion<N>) {
        *self = *self * rhs;
    }
}

impl<N: Number> DivAssign<Quaternion<N>> for Quaternion<N> {
    fn div_assign(&mut self, rhs: Quaternion<N>) {
        *self = *self / rhs;
    }
}