use std::sync::{Arc, Mutex};

use crate::linear_algebra::vector::types::{Vector3F64, Vector4F64};

pub mod flags;
pub mod imp;

#[derive(Debug)]
pub struct RigidbodyInner {
    position: Vector3F64,
    rotation: Vector3F64,
    mass: f64,

    velocity: Vector3F64,
    flags: u8
}

#[derive(Clone)]
pub struct Rigidbody(Arc<Mutex<RigidbodyInner>>);