use crate::linear_algebra::vector::types::Vector3F64;
use crate::physics::collider::bounds::Bounds;
use crate::physics::collider::Collider;
use crate::physics::rigidbody_handle::RigidbodyHandle;

pub struct BoxCollider {
    size: Vector3F64,
    center: Vector3F64,

    _bounds: Bounds,
    rigidbody_handle: Option<RigidbodyHandle>
}

impl BoxCollider {

}

impl Collider for BoxCollider {
    fn update(&mut self) {
        if let Some(rigid) = self.rigidbody() {
            self._bounds = Bounds {
                center: rigid.rigidbody.position() + self.center,
                extents: self.size / 2_f64,
            };
        }
    }

    fn check_collision(&self, b: &dyn Collider) -> bool {
        false
    }

    fn point_of_contact(&self, b: &dyn Collider) -> Option<Vector3F64> {
        None
    }

    fn rigidbody(&self) -> Option<&RigidbodyHandle> {
        self.rigidbody_handle.as_ref()
    }
}