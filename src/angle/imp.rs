use crate::angle::Angle;
use crate::number::Number;

impl <N: Number> Angle<N> {
    pub fn to_radians(self) -> Self {
        match self {
            Angle::Radians(_) => self,
            Angle::Degrees(d) => Self::Radians(d.rad())
        }
    }

    pub fn to_degrees(self) -> Self {
        match self {
            Angle::Radians(r) => Self::Degrees(r.deg()),
            Angle::Degrees(_) => self
        }
    }

    pub fn take(self) -> N {
        match self {
            Angle::Radians(r) => r,
            Angle::Degrees(d) => d
        }
    }
}