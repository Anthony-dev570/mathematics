use crate::linear_algebra::vector::types::{Vector2, Vector3F32};
use crate::linear_algebra::vector::Vector;
use crate::shared::traits::number::Number;

pub mod shape;
pub mod triangle;
pub mod curve;
pub mod bezier;
pub mod uv_sphere;

#[derive(Debug, Clone)]
pub struct Geometry<const L: usize, N: Number> {
    pub vertices: Vec<Vector<L, N>>,
    pub normals: Option<Vec<Vector<L, N>>>,
    pub uv: Vec<Vector2<N>>,
    pub indices: Vec<u32>,
}