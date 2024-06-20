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
    use image::{ColorType, GenericImage, Rgb, RgbImage};
    use image::ColorType::Rgba8;
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
    fn test_curves() {
        let p0 = Vector2F32::new([0_f32; 2]);
        let p1 = Vector2F32::new([10_f32, 20_f32]);
        let p2 = Vector2F32::new([20_f32, 0_f32]);

        let curve = Curve::Quadratic {
            p0,
            p1,
            p2
        };

        let points = curve.points(0.1);

        let mut img = RgbImage::new(21, 20);

        for point in points {
            let px = point.x() as u32;
            let py = point.y() as u32;
            println!("{:?}", (px, py));
            if px < 21 && py < 20 {
                img.put_pixel(point.x() as u32, point.y() as u32, Rgb([255; 3]));
            }
        }

        img.save("graph.png").unwrap();
    }
}
