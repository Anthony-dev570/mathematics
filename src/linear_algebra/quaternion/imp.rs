use std::ops::{Mul, Neg};

use crate::linear_algebra::euler_angles::EulerAngles;
use crate::linear_algebra::matrix::types::Mat3;
use crate::linear_algebra::quaternion::{Quaternion, QuaternionConstructor};
use crate::linear_algebra::vector::types::Vector3;
use crate::linear_algebra::vector::Vector;
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

impl<N: Number + Neg<Output=N>> Mul<Vector3<N>> for Quaternion<N> {
    type Output = Vector3<N>;

    fn mul(self, rhs: Vector3<N>) -> Self::Output {
        let q = Self::new(QuaternionConstructor::Pure {
            xyz: rhs,
        });
        (self * q * self.conjugate()).xyz
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

    pub fn norm(&self) -> N {
        (self.xyz[0].num_pow(N::TWO) + self.xyz[1].num_pow(N::TWO) + self.xyz[2].num_pow(N::TWO) + self.w.num_pow(N::TWO)).num_sqrt()
    }

    pub fn unit_quaternion(&self) -> Self {
        let norm = self.norm();
        Self {
            xyz: self.xyz / norm,
            w: self.w / norm,
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
}