use crate::linear_algebra::euler_angles::EulerAngles;
use crate::linear_algebra::quaternion::Quaternion;
use crate::linear_algebra::vector::types::Vector3;
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

impl <N: Number> Quaternion<N> {
    pub fn to_euler_angle<E: EulerAngles<N>>(self) -> E {
        let (mut roll, mut pitch, mut yaw) = (N::ZERO, N::ZERO, N::ZERO);

        let (one, two) = (N::ONE, N::TWO);

        let (w, x, y, z) = (self.w, self.xyz[0], self.xyz[1], self.xyz[2]);

        let sinr_cosp = two * (w * x + y * z);
        let cosr_cosp = one - two * (x * x + y * y);
        roll = sinr_cosp.arc_tan2(cosr_cosp);

        let sinp = (one + two * (w * y - x * z)).num_sqrt();
        let cosp = (one - two * (w * y - x * z)).num_sqrt();
        pitch = two * sinp.arc_tan2(cosp) - N::PI / two;

        let siny_cosp = two * (w * z + x * y);
        let cosy_cosp = one - two * (y * y + z * z);
        yaw = siny_cosp.arc_tan2(cosy_cosp);

        E::from_xyz(Angle::radians(roll), Angle::radians(pitch), Angle::radians(yaw))
    }

    pub fn xyz(&self) -> &Vector3<N> {
        &self.xyz
    }
    pub fn w(&self) -> &N {
        &self.w
    }
}