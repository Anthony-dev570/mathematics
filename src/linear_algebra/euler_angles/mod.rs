use crate::linear_algebra::quaternion::{Quaternion, QuaternionConstructor};
use crate::linear_algebra::vector::Vector;
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

pub mod proper_euler_angles;
pub mod principle_euler_angles;

pub trait EulerAngles<N: Number> {
    fn from_xyz(x: Angle<N>, y: Angle<N>, z: Angle<N>) -> Self;
    fn x(&self) -> &Angle<N>;
    fn y(&self) -> &Angle<N>;
    fn z(&self) -> &Angle<N>;

    fn to_quaternion(&self) -> Quaternion<N> {
        let half = N::ONE / N::TWO;

        let (roll, pitch, yaw) = (
            self.x().to_radians().take() * half,
            self.y().to_radians().take() * half,
            self.z().to_radians().take() * half
        );

        let (cr, sr) = (roll.cosine(), roll.sine());
        let (cp, sp) = (pitch.cosine(), pitch.sine());
        let (cy, sy) = (yaw.cosine(), yaw.sine());

        let xyz = Vector::new([
            sr * cp * cy - cr * sp * sy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy
        ]);
        let w = cr * cp * cy + sr * sp * sy;

        Quaternion::new(QuaternionConstructor::Default {
            xyz,
            w,
        })
    }
}