use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

impl <N: Number> Angle<N> {

    pub fn degrees(degrees: N) -> Self {
        Self::Degrees(degrees)
    }

    pub fn radians(radians: N) -> Self {
        Self::Radians(radians)
    }

    pub fn is_radians(&self) -> bool {
        match self {
            Angle::Radians(_) => true,
            Angle::Degrees(_) => false
        }
    }

    pub fn is_degrees(&self) -> bool {
        !self.is_radians()
    }

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