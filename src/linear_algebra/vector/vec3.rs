use crate::linear_algebra::vector::types::{Vector2, Vector3};
use crate::shared::traits::number::Number;

impl<N: Number> Vector3<N> {
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

    pub fn cross(&self, b: &Self) -> Self {
        let (ax, ay, az) = self.xyz();
        let (bx, by, bz) = b.xyz();

        Self([
            ay * bz - az * by,
            az * bx - ax * bz,
            ax * by - ay * bx
        ])
    }
}