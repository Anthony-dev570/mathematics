#![allow(warnings)]
#![feature(generic_const_exprs)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod shared;
pub mod algebra;
pub mod linear_algebra;
pub mod chemistry;
pub mod geometry;
pub mod color;

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::geometry::shape::Shape;
    use crate::geometry::triangle::triangle2d::Triangle2D;
    use crate::linear_algebra::euler_angles::EulerAngles;
    use crate::linear_algebra::euler_angles::principle_euler_angles::PrincipleEulerAngles;
    use crate::linear_algebra::vector::types::{Vector2F32, Vector3};
    use crate::shared::angle::Angle::Degrees;
    use crate::shared::traits::lerp::Lerp;

    #[test]
    fn test_geometry() {
        let triangle_2d = Triangle2D::FromVertices {
            a: Vector2F32::new([0.0; 2]),
            b: Vector2F32::new([1.0, 0.0]),
            c: Vector2F32::new([0.0, 1.0]),
        };
        let a = triangle_2d.area();
        println!("{}", a);
    }

    #[test]
    fn test_color() {
        let color = Color::HSL {
            hue: Degrees(50_f32),
            saturation: 10_f32,
            lightness: 10_f32,
        };

        println!("{:?}", color.to_rgb());
    }

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
