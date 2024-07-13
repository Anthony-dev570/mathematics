use crate::linear_algebra::vector::types::Vector3F64;
use crate::physics::collider::Collider;
use crate::physics::rigidbody_handle::RigidbodyHandle;

pub struct SphereCollider {
    radius: f64,
    rigidbody_handle: Option<RigidbodyHandle>,
}

impl SphereCollider {
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
            rigidbody_handle: None,
        }
    }

    pub fn set_handle(&mut self, rigidbody_handle: RigidbodyHandle) {
        self.rigidbody_handle = Some(rigidbody_handle);
    }
}

impl Collider for SphereCollider {
    fn update(&mut self) {

    }

    fn check_collision(&self, b: &dyn Collider) -> bool {
        if let Some(a_rigid) = self.rigidbody() {
            if let Some(b_rigid) = b.rigidbody() {
                if let Some(b_sphere) = b.downcast_ref::<SphereCollider>() {
                    let d = a_rigid.rigidbody.position().distance(&b_rigid.rigidbody.position());
                    return (self.radius + b_sphere.radius) >= d;
                }
            }
        }
        false
    }

    fn point_of_contact(&self, b: &dyn Collider) -> Option<Vector3F64> {
        if let Some(a_rigid) = self.rigidbody() {
            if let Some(b_rigid) = b.rigidbody() {
                if let Some(b_sphere) = b.downcast_ref::<SphereCollider>() {
                    let c = b_rigid.rigidbody.position() - a_rigid.rigidbody.position();
                    let d = c.magnitude();
                    if (self.radius + b_sphere.radius) >= d {
                        return Some(c.normalize() * self.radius);
                    }
                }
            }
        }
        None
    }

    fn rigidbody(&self) -> Option<&RigidbodyHandle> {
        self.rigidbody_handle.as_ref()
    }
}

impl Default for SphereCollider {
    fn default() -> Self {
        Self {
            radius: 1.0,
            rigidbody_handle: None,
        }
    }
}