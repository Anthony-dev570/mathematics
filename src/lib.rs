#![allow(warnings)]
#![feature(generic_const_exprs)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod shared;
pub mod algebra;
pub mod linear_algebra;
pub mod chemistry;

#[cfg(test)]
mod tests {
    use crate::linear_algebra::euler_angles::EulerAngles;
    use crate::linear_algebra::euler_angles::principle_euler_angles::PrincipleEulerAngles;
    use crate::linear_algebra::vector::types::Vector3;
    use crate::shared::angle::Angle::Degrees;
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
        let v = Vector3::new([1_f32, 0_f32, 0_f32]);
        let e = PrincipleEulerAngles {
            roll: Degrees(90_f32),
            pitch: Degrees(0_f32),
            yaw: Degrees(0_f32),
        }.to_quaternion();
        println!("{:?}", e);
        let v_prime = e * v;

        println!("{:?}", v_prime);
    }
}
