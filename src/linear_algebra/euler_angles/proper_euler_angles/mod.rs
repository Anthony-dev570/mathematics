use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

pub mod imp;

pub enum ProperEulerAngles<N: Number> {
    AlphaBetaGamma {
        alpha: Angle<N>,
        beta: Angle<N>,
        gamma: Angle<N>
    },
    PsiThetaPhi {
        psi: Angle<N>,
        theta: Angle<N>,
        phi: Angle<N>
    }
}