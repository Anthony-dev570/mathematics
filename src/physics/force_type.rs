#[derive(Debug, Clone, Copy)]
pub enum ForceType {
    ///Add a continuous force to the object using the object's mass.
    Force,
    ///Add a continuous force to the object ignoring the object's mass.
    Acceleration,
    ///Adds an instant force impulse to the object using its mass.
    Impulse,
    ///Add an instant velocity change to the object ignoring its mass.
    VelocityChange
}