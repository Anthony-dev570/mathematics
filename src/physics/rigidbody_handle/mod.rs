use crate::physics::rigidbody::Rigidbody;

#[derive(Clone)]
pub struct RigidbodyHandle {
    pub(crate) id: u32,
    pub(crate) rigidbody: Rigidbody
}