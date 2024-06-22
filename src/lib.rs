#![allow(warnings)]
#![feature(generic_const_exprs)]
#![feature(const_fn_floating_point_arithmetic)]

use crate::geometry::curve::Curve;
use crate::linear_algebra::vector::types::Vector2F32;

pub mod shared;
pub mod algebra;
pub mod linear_algebra;
pub mod chemistry;
pub mod geometry;
pub mod color;

pub fn do_test() {
    Curve::Linear {
        p0: Vector2F32::default(),
        p1: Vector2F32::default(),
    };
}

#[cfg(test)]
mod tests {
    use image::{GenericImage, Rgb, RgbImage};

    use crate::color::Color;
    use crate::geometry::curve::Curve;
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

    #[test]
    fn test_inverse_curve() {
        let p0 = Vector2F32::new([0_f32; 2]);
        let p1 = Vector2F32::new([0_f32, 100_f32]);
        let p2 = Vector2F32::new([100_f32, 0.0]);
        let p3 = Vector2F32::new([100_f32; 2]);

        let curve = Curve::Cubic {
            p0,
            p1,
            p2,
            p3,
        };

        let p = curve.interpolate(0.5);

        let p0x = p0.x();
        let p1x = p1.x();
        let p2x = p2.x();
        let p3x = p3.x();
        let px = p.x();

        let a = p3x - 3_f32 * p2x + 3_f32 * p1x - p0x;
        let b = 3_f32 * p2x - 6_f32 * p1x + 3_f32 * p0x;
        let c = 3_f32 * p1x - 3_f32 * p0x;
        let d = p0x - px;


        let p = -b / (3_f32 * a);
        let q = (p * p * p) + (b * c - 3_f32 * a * d) / (6_f32 * (a * a));
        let r = c / (3_f32 * a);

        println!("{:?}", (a, b, c, d));

        println!("{:?}", (p, q, r));

        //q + [q2 + (r-p2)3]1/2

        let q2 = q * q;
        let r_p2_3 = (r - p * p) * (r - p * p) * (r - p * p);

        println!("{:?}", r_p2_3);

        let inner = (
            q2 + r_p2_3
        ).powf(0.5);

        println!("inner: {inner}");

        let a = (q + inner).powf(0.33);
        let b = (q - inner).powf(0.33) + p;

        println!("{:?}", (a, b));
    }

    #[test]
    fn test_curves() {
        let p0 = Vector2F32::new([0_f32; 2]);
        let p1 = Vector2F32::new([0_f32, 50_f32]);
        let p2 = Vector2F32::new([99_f32, 50.0]);
        let p3 = Vector2F32::new([99_f32, 99_f32]);

        let curve = Curve::Cubic {
            p0,
            p1,
            p2,
            p3,
        };

        let points = curve.points(0.05);

        let mut img = RgbImage::new(100, 100);

        for point in points {
            let px = point.x() as u32;
            let py = point.y() as u32;

            img.put_pixel(point.x() as u32, point.y() as u32, Rgb([255; 3]));
        }

        img.put_pixel(p1.x() as u32, p1.y() as u32, Rgb([255, 0, 0]));
        img.put_pixel(p2.x() as u32, p2.y() as u32, Rgb([255, 0, 0]));

        img.save("graph.png").unwrap();
    }
}
