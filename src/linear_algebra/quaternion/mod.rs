use crate::constructor;
use crate::linear_algebra::vector::types::Vector3;
use crate::shared::traits::number::Number;

pub mod imp;

constructor!(
    <N> Quaternion where N: Number {
        Default(xyz: Vector3<N>, w: N) {
            Quaternion {
                xyz,
                w,
            }
        },
        Pure(xyz: Vector3<N>) {
            Quaternion {
                xyz,
                w: N::ZERO
            }
        }
    }
);

#[derive(Debug, Clone, Copy)]
pub struct Quaternion<N: Number> {
    xyz: Vector3<N>,
    w: N
}