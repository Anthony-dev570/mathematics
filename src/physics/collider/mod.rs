use downcast_rs::{Downcast, impl_downcast};
use crate::linear_algebra::vector::types::Vector3F64;
use crate::physics::rigidbody_handle::RigidbodyHandle;

pub mod sphere_collider;
pub mod box_collider;
pub mod bounds;

impl_downcast!(Collider);

pub trait Collider: Downcast {
    fn update(&mut self);
    fn check_collision(&self, b: &dyn Collider) -> bool;
    fn point_of_contact(&self, b: &dyn Collider) -> Option<Vector3F64>;
    fn rigidbody(&self) -> Option<&RigidbodyHandle>;
}