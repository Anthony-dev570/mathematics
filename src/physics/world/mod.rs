use std::collections::HashMap;
use std::time::Instant;
use crate::linear_algebra::vector::types::{Vector3F32, Vector3F64};
use crate::physics::rigidbody_handle::RigidbodyHandle;

pub mod imp;

pub struct World {
    gravity: Vector3F64,
    rigidbody_handles: HashMap<u32, RigidbodyHandle>,
    rigidbody_increment: u32,
    instant: Instant,
    clock: Instant,
    fc: usize,
    skip_ticks: u32,
    frame_rate: u8
}