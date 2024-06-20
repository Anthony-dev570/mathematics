use crate::linear_algebra::vector::types::{Vector2, Vector3};
use crate::shared::traits::number::Number;

impl<N: Number> Vector3<N> {
    pub fn vec2(self) -> Vector2<N> {
        Vector2::new([self.0[0], self.0[1]])
    }
}