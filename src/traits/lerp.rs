pub trait Lerp {
    fn lerp(&self, b: &Self, t: f32) -> Self;
}