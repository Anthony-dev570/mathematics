use crate::linear_algebra::euler_angles::EulerAngles;
use crate::linear_algebra::euler_angles::proper_euler_angles::ProperEulerAngles;
use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

impl <N: Number> ProperEulerAngles<N> {
    
}

impl <N: Number> EulerAngles<N> for ProperEulerAngles<N> {
    fn from_xyz(x: Angle<N>, y: Angle<N>, z: Angle<N>) -> Self {
        Self::AlphaBetaGamma {
            alpha: x,
            beta: y,
            gamma: z,
        }
    }

    fn x(&self) -> &Angle<N> {
        match self {
            ProperEulerAngles::AlphaBetaGamma { alpha, .. } => alpha,
            ProperEulerAngles::PsiThetaPhi { psi, .. } => psi
        }
    }

    fn y(&self) -> &Angle<N> {
        match self {
            ProperEulerAngles::AlphaBetaGamma { beta, .. } => beta,
            ProperEulerAngles::PsiThetaPhi { theta, .. } => theta
        }
    }

    fn z(&self) -> &Angle<N> {
        match self {
            ProperEulerAngles::AlphaBetaGamma { gamma, .. } => gamma,
            ProperEulerAngles::PsiThetaPhi { phi, .. } => phi
        }
    }
}