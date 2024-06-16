use std::ops::{Div, Mul, Neg};

use crate::linear_algebra::euler_angles::EulerAngles;
use crate::linear_algebra::matrix::types::Mat3;
use crate::linear_algebra::quaternion::Quaternion;
use crate::linear_algebra::vector::types::Vector3;
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

pub mod operations;

impl<N: Number> Quaternion<N> {
    pub fn to_euler_angle<E: EulerAngles<N>>(self) -> E {
        let (mut roll, mut pitch, mut yaw) = (N::ZERO, N::ZERO, N::ZERO);

        let (one, two) = (N::ONE, N::TWO);

        let (w, x, y, z) = (self.w, self.xyz[0], self.xyz[1], self.xyz[2]);

        let sinr_cosp = two * (w * x + y * z);
        let cosr_cosp = one - two * (x * x + y * y);
        roll = sinr_cosp.arc_tan2(cosr_cosp);

        let sinp = (one + two * (w * y - x * z)).num_sqrt();
        let cosp = (one - two * (w * y - x * z)).num_sqrt();
        pitch = two * sinp.arc_tan2(cosp) - N::PI / two;

        let siny_cosp = two * (w * z + x * y);
        let cosy_cosp = one - two * (y * y + z * z);
        yaw = siny_cosp.arc_tan2(cosy_cosp);

        E::from_xyz(Angle::radians(roll), Angle::radians(pitch), Angle::radians(yaw))
    }

    pub fn conjugate(&self) -> Self where N: Neg<Output=N> {
        Self {
            xyz: -self.xyz,
            w: self.w,
        }
    }

    pub fn magnitude(&self) -> N {
        (self.xyz[0].num_pow(N::TWO) + self.xyz[1].num_pow(N::TWO) + self.xyz[2].num_pow(N::TWO) + self.w.num_pow(N::TWO)).num_sqrt()
    }

    pub fn norm(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            xyz: self.xyz / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn xyz(&self) -> &Vector3<N> {
        &self.xyz
    }
    pub fn w(&self) -> &N {
        &self.w
    }

    pub fn wxyz(&self) -> [N; 4] {
        [self.w, self.xyz[0], self.xyz[1], self.xyz[2]]
    }

    pub fn components(&self) -> (N, N, N, N) {
        let wxyz = self.wxyz();
        (wxyz[0], wxyz[1], wxyz[2], wxyz[3])
    }

    pub fn to_rotation_matrix(&self) -> Mat3<N> {
        let (q0, q1, q2, q3) = self.components();

        let a = N::TWO * (q0.num_pow(N::TWO) + q1.num_pow(N::TWO)) - N::ONE;
        let b = N::TWO * (q1 * q2 + q0 * q3);
        let c = N::TWO * (q1 * q3 - q0 * q2);

        let d = N::TWO * (q1 * q2 - q0 * q3);
        let e = N::TWO * (q0.num_pow(N::TWO) + q2.num_pow(N::TWO)) - N::ONE;
        let f = N::TWO * (q2 * q3 + q0 * q1);

        let g = N::TWO * (q1 * q3 + q0 * q2);
        let h = N::TWO * (q2 * q3 - q0 * q1);
        let i = N::TWO * (q0.num_pow(N::TWO) + q3.num_pow(N::TWO)) - N::ONE;

        Mat3::new([
            [a, b, c],
            [d, e, f],
            [g, h, i]
        ])
    }

    pub fn inverse(&self) -> Self where N: Neg<Output=N> {
        let conjugate = self.conjugate();
        conjugate / (*self * conjugate)
    }

    pub fn slerp(&self, b: &Self, t: N) -> Quaternion<N> where N: Neg<Output=N> {
        let (one, two) = (*self, *b);
        (two * one.inverse()) * t * one
    }
}