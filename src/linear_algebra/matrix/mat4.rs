use std::ops::Neg;
use crate::linear_algebra::matrix::types::{Mat3, Mat4};
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

impl<N: Number> Mat4<N> {
    crate::to_mat! {
        N, 4 => 2, 3, 5, 6, 7, 8, 9
    }

    pub fn rotation_x(x: Angle<N>) -> Mat4<N> where N: Neg<Output=N> {
        Mat3::rotation_x(x).to_mat4()
    }

    pub fn rotation_y(y: Angle<N>) -> Mat4<N> where N: Neg<Output=N> {
        Mat3::rotation_y(y).to_mat4()
    }

    pub fn rotation_z(z: Angle<N>) -> Mat4<N> where N: Neg<Output=N> {
        Mat3::rotation_z(z).to_mat4()
    }

    pub fn orthographic(
        left: N,
        right: N,
        bottom: N,
        top: N,
        near: N,
        far: N,
    ) -> Self where N: Neg<Output=N> {
        /*Self([
            [N::TWO / (right - left), N::ZERO, N::ZERO, N::ZERO],
            [N::ZERO, N::TWO / (top - bottom), N::ZERO, N::ZERO],
            [N::ZERO, N::ZERO, -N::TWO / (far - near), N::ZERO],
            [-(right + left) / (right - left), -(top + bottom) / (top - bottom), -(far + near / far - near), N::ONE],
        ])*/

        let mut out = Self::identity();
        let two = N::TWO;

        out[0][0] = two / (right - left);
        out[0][3] = -(right + left) / (right - left);
        out[1][1] = two / (top - bottom);
        out[1][3] = -(top + bottom) / (top - bottom);
        out[2][2] = -two / (far - near);
        out[2][3] = -(far + near) / (far - near);

        out
    }
}