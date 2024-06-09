pub trait Lerp {
    ///Performs a linear interpolation between two states (self and b), at a given ratio between [0, 1] denoted by t.
    ///<br>See <a href='https://en.wikipedia.org/wiki/Linear_interpolation'>Linear Interpolation</a>
    fn lerp(&self, b: &Self, t: f32) -> Self;
}