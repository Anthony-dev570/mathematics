use std::ops::Neg;
use crate::shared::angle::Angle;
use crate::linear_algebra::matrix::types::Mat3;

use crate::shared::traits::number::Number;

impl<N: Number> Mat3<N> {
    crate::to_mat! {
        N, 3 => 2, 4, 5, 6, 7, 8, 9
    }

    pub fn rotation_x(theta: Angle<N>) -> Self where N: Neg<Output=N> {
        let r = theta.to_radians().take();
        let (o, z) = (N::ONE, N::ZERO);
        let (c, s) = r.cos_sin();

        Self([
            [o, z, z],
            [z, c, s],
            [z, -s, c]
        ])
    }

    pub fn rotation_y(theta: Angle<N>) -> Self where N: Neg<Output=N> {
        let r = theta.to_radians().take();
        let (o, z) = (N::ONE, N::ZERO);
        let (c, s) = r.cos_sin();

        Self([
            [c, z, -s],
            [z, o, z],
            [s, z, c]
        ])
    }

    pub fn rotation_z(theta: Angle<N>) -> Self where N: Neg<Output=N> {
        let r = theta.to_radians().take();
        let (o, z) = (N::ONE, N::ZERO);
        let (c, s) = r.cos_sin();

        Self([
            [c, s, z],
            [-s, c, z],
            [z, z, o]
        ])
    }
}