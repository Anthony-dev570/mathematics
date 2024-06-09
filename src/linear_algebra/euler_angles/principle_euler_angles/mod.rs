use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

pub mod imp;

#[derive(Debug, Clone, Copy)]
pub struct PrincipleEulerAngles<N: Number> {
    pub roll: Angle<N>,
    pub pitch: Angle<N>,
    pub yaw: Angle<N>
}