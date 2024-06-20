use crate::linear_algebra::vector::types::{Vector2, Vector3};
use crate::shared::traits::number::Number;

impl <N: Number> Vector2<N> {
    pub fn vec3(self) -> Vector3<N> {
        Vector3::new([self.0[0], self.0[1], N::ZERO])
    }
}