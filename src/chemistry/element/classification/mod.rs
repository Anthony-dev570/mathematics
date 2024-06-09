pub mod imp;

#[derive(Debug, Clone, Copy)]
pub enum Classification {
    Alkali,
    AlkalineEarthMetal,
    Lanthanoid,
    Actinoid,
    TransitionMetal,
    PostTransitionMetal,

    Metaloid,
    ReactiveNonmetal,
    NobleGas
}