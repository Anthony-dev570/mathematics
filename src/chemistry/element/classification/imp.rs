use crate::chemistry::element::classification::Classification;

impl Classification {
    pub fn is_metal(&self) -> bool {
        match self {
            Classification::Metaloid | Classification::ReactiveNonmetal | Classification::NobleGas => false,
            _ => true
        }
    }

    pub fn is_non_metal(&self) -> bool {
        match self {
            Classification::ReactiveNonmetal | Classification::NobleGas => true,
            _ => false
        }
    }
}