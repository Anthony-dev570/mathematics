use std::ops::Neg;
use crate::linear_algebra::matrix::types::{Mat3, Mat4};
use crate::linear_algebra::vector::types::Vector3;
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

impl<N: Number> Mat4<N> {
    crate::to_mat! {
        N, 4 => 2, 3, 5, 6, 7, 8, 9
    }

    pub fn rotation_x(x: Angle<N>) -> Mat4<N>
    where
        N: Neg<Output=N>,
    {
        Mat3::rotation_x(x).to_mat4()
    }

    pub fn rotation_y(y: Angle<N>) -> Mat4<N>
    where
        N: Neg<Output=N>,
    {
        Mat3::rotation_y(y).to_mat4()
    }

    pub fn rotation_z(z: Angle<N>) -> Mat4<N>
    where
        N: Neg<Output=N>,
    {
        Mat3::rotation_z(z).to_mat4()
    }

    pub fn look_at(
        target: Vector3<N>,
        eye: Vector3<N>,
        up: Vector3<N>,
    ) -> Mat4<N> where
        N: Neg,
    {
        let fwd = (target - eye).normalize();
        let side = fwd.cross(&up).normalize();
        let up = side.cross(&fwd).normalize();

        let (sx, sy, sz) = side.xyz();
        let (ux, uy, uz) = up.xyz();
        let (fx, fy, fz) = fwd.xyz();

        Self([
            [sx, ux, -fx, N::ZERO],
            [sy, uy, -fy, N::ZERO],
            [sz, uz, -fz, N::ZERO],
            [-side.dot(&eye), -up.dot(&eye), -fwd.cross(&eye), N::ZERO]
        ])
    }

    pub fn perspective(
        aspect_ratio: N,
        fov: Angle<N>,
        near: N,
        far: N,
    ) -> Self where
        N: Neg,
    {
        let fov = (fov.to_radians().take() / N::TWO).tangent();
        Self([
            [N::ONE / (aspect_ratio * fov), N::ZERO, N::ZERO, N::ZERO],
            [N::ZERO, N::ONE / fov, N::ZERO, N::ZERO],
            [N::ZERO, N::ZERO, -(far + near) / (far - near), -N::ONE],
            [N::ZERO, N::ZERO, -(N::TWO * far * near) / (far - near), N::ZERO]
        ])
    }

    pub fn orthographic(
        left: N,
        right: N,
        bottom: N,
        top: N,
        near: N,
        far: N,
    ) -> Self where
        N: Neg<Output=N>,
    {
        let mut out = Self::identity();
        let two = N::TWO;

        out[0][0] = two / (right - left);
        out[3][0] = -(right + left) / (right - left);
        out[1][1] = two / (top - bottom);
        out[3][1] = -(top + bottom) / (top - bottom);
        out[2][2] = -two / (far - near);
        out[3][2] = -(far + near) / (far - near);

        out
    }

    pub fn scale(scale: Vector3<N>) -> Self {
        Mat3::scale(scale).to_mat4()
    }

    pub fn translation(translation: Vector3<N>) -> Self {
        let (z, o) = (N::ZERO, N::ONE);
        Self([
            [o, z, z, z],
            [z, o, z, z],
            [z, z, o, z],
            [translation[0], translation[1], translation[2], o]
        ])
    }
}