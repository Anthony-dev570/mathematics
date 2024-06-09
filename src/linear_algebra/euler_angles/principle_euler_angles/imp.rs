use crate::linear_algebra::euler_angles::EulerAngles;
use crate::linear_algebra::euler_angles::principle_euler_angles::PrincipleEulerAngles;
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

impl <N: Number> EulerAngles<N> for PrincipleEulerAngles<N> {
    fn from_xyz(x: Angle<N>, y: Angle<N>, z: Angle<N>) -> Self {
        Self {
            roll: x,
            pitch: y,
            yaw: z,
        }
    }

    fn x(&self) -> &Angle<N> {
        &self.roll
    }

    fn y(&self) -> &Angle<N> {
        &self.pitch
    }

    fn z(&self) -> &Angle<N> {
        &self.yaw
    }
}