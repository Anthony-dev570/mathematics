#![allow(warnings)]
#![feature(generic_const_exprs)]

pub mod number;
pub mod endian;
pub mod vector;
pub mod matrix;
pub mod angle;
pub mod traits;

#[cfg(test)]
mod tests {
    use crate::matrix::types::Mat2;

    #[test]
    fn it_works() {
        let m = Mat2::new([[3.25342343; 2]; 2]);
        let m6 = m.to_mat3();
        let m2 = m6.to_mat2().to_mat6();
        println!("{}", m2);
    }
}
