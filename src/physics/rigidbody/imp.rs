use std::fmt::{Debug, Formatter, Write};
use std::sync::{Arc, Mutex};
use crate::linear_algebra::vector::types::Vector3F64;
use crate::physics::force_type::ForceType;
use crate::physics::rigidbody::{Rigidbody, RigidbodyInner};
use crate::physics::rigidbody::flags::RigidbodyFlags;

impl Rigidbody {
    pub fn update(&mut self, delta: f64, gravity: &Vector3F64) {
        let inner = &mut self.0.lock().unwrap();
        if !(inner.flags & RigidbodyFlags::UseKinematics > 0) {
            return;
        }

        if inner.flags & RigidbodyFlags::UseGravity > 0 {
            inner.velocity += *gravity * delta;
        }

        let velocity = inner.velocity * delta;

        inner.position += velocity;
    }

    pub fn add_force(&self, force: Vector3F64, force_type: ForceType, delta: f64) {
        let mut inner = self.0.lock().unwrap();
        match force_type {
            ForceType::Force => {}
            ForceType::Acceleration => {
                inner.velocity += force * delta;
            }
            ForceType::Impulse => {
                let mass = inner.mass;
                inner.velocity += force * mass;
            }
            ForceType::VelocityChange => {
                inner.velocity = force;
            }
        }
    }

    pub fn position(&self) -> Vector3F64 {
        self.0.lock().unwrap().position
    }

    pub fn set_position(&self, position: Vector3F64) {
        self.0.lock().unwrap().position = position;
    }
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(RigidbodyInner {
            position: Default::default(),
            rotation: Default::default(),
            mass: 1.0,
            velocity: Default::default(),
            flags: RigidbodyFlags::DEFAULT,
        })))
    }
}

impl Debug for Rigidbody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.lock().unwrap()))
    }
}