#![allow(warnings)]
#![feature(generic_const_exprs)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod shared;
pub mod algebra;
pub mod linear_algebra;
pub mod chemistry;

#[cfg(test)]
mod tests {
    use crate::linear_algebra::matrix::Matrix;
    use crate::shared::traits::lerp::Lerp;

    #[test]
    fn test_lerp() {
        let a = 5;
        let b = 10;
        let c = a.lerp(&b, 0.5);
        println!("{:?}", (a, b, c));
    }

    #[test]
    fn it_works() {
        let mut m = Matrix::new([[0.0; 2]; 3]);

        println!("{}\nvs\n{}", m, m.transpose());
    }
}
