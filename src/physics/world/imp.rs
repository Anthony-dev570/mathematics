use std::time::Instant;
use crate::linear_algebra::vec3;

use crate::physics::rigidbody::Rigidbody;
use crate::physics::rigidbody_handle::RigidbodyHandle;
use crate::physics::world::World;

impl World {
    pub fn register_rigidbody(&mut self, rigidbody: Rigidbody) -> RigidbodyHandle {
        let handle = RigidbodyHandle {
            id: self.rigid_inc(),
            rigidbody,
        };

        let out = handle.clone();
        self.rigidbody_handles.insert(handle.id, handle);
        out
    }

    pub fn create_rigidbody(&mut self) -> RigidbodyHandle {
        self.register_rigidbody(Rigidbody::default())
    }

    pub fn update(&mut self) {
        let velocity = self.gravity;
        let (millis, secs) = {
            let time = self.instant.elapsed();
            (time.as_millis() as u32, time.as_secs_f64())
        };

        if millis >= self.skip_ticks {
            self.fc += 1;
            self.instant = Instant::now();
            for (_, v) in &mut self.rigidbody_handles {
                v.rigidbody.update(secs, &velocity);
            }
        }
        if self.clock.elapsed().as_secs() >= 1 {
            self.fc = 0;
            self.clock = Instant::now();
        }
    }

    fn rigid_inc(&mut self) -> u32 {
        let id = self.rigidbody_increment;
        self.rigidbody_increment += 1;
        id
    }
}

impl Default for World {
    fn default() -> Self {
        let now = Instant::now();
        let frame_rate = 60;
        let skip_ticks = 1000 / frame_rate as u32;
        Self {
            gravity: vec3(0.0, -9.81, 0.0),
            rigidbody_handles: Default::default(),
            rigidbody_increment: 0,
            instant: now,
            clock: now,
            fc: 0,
            skip_ticks,
            frame_rate,
        }
    }
}