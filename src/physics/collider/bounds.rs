use crate::linear_algebra::vector::types::Vector3F64;

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub center: Vector3F64,
    pub extents: Vector3F64
}