use crate::linear_algebra::scalar::Scalar;
use crate::linear_algebra::vector::types::{Vector2, Vector3, Vector4};
use crate::shared::traits::number::Number;

impl<N: Scalar> Vector4<N> {
    pub const DOWN: Self = Self([N::ZERO, N::NEG_ONE, N::ZERO, N::ZERO]);
    pub const BACKWARD: Self = Self([N::ZERO, N::ZERO, N::NEG_ONE, N::ZERO]);
    pub const LEFT: Self = Self([N::NEG_ONE, N::ZERO, N::ZERO, N::ZERO]);
}

impl<N: Number> Vector4<N> {
    pub const UP: Self = Self([N::ZERO, N::ONE, N::ZERO, N::ZERO]);
    pub const FORWARD: Self = Self([N::ZERO, N::ZERO, N::ONE, N::ZERO]);
    pub const RIGHT: Self = Self([N::ONE, N::ZERO, N::ZERO, N::ZERO]);

    pub fn vec2(self) -> Vector2<N> {
        Vector2::new([self.0[0], self.0[1]])
    }

    pub fn x(&self) -> N {
        self[0]
    }

    pub fn y(&self) -> N {
        self[1]
    }

    pub fn z(&self) -> N {
        self[2]
    }

    pub fn w(&self) -> N { self[3] }

    pub fn xy(&self) -> (N, N) {
        (self.x(), self.y())
    }

    pub fn xz(&self) -> (N, N) {
        (self.x(), self.z())
    }

    pub fn yz(&self) -> (N, N) {
        (self.y(), self.z())
    }

    pub fn xyz(&self) -> (N, N, N) {
        (self.x(), self.y(), self.z())
    }
    pub fn xyzw(&self) -> (N, N, N, N) {
        (self.x(), self.y(), self.z(), self.w())
    }

    pub fn vec3(self) -> Vector3<N> {
        Vector3::new([self.x(), self.y(), self.z()])
    }
}