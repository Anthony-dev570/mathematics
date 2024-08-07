#![feature(const_trait_impl)]
#![allow(warnings)]
#![feature(generic_const_exprs)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod shared;
pub mod algebra;
pub mod linear_algebra;
pub mod chemistry;
pub mod geometry;
pub mod color;
pub mod physics;

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::thread::spawn;
    use std::time::Instant;

    use image::{ColorType, GenericImage, Rgb, Rgba, RgbImage};

    use crate::algebra::summation;
    use crate::color::Color;
    use crate::geometry::curve::Curve;
    use crate::geometry::shape::Shape;
    use crate::geometry::triangle::triangle2d::Triangle2D;
    use crate::geometry::uv_sphere::UVSphere;
    use crate::linear_algebra::euler_angles::EulerAngles;
    use crate::linear_algebra::euler_angles::principle_euler_angles::PrincipleEulerAngles;
    use crate::linear_algebra::matrix::Matrix;
    use crate::linear_algebra::matrix::types::Mat4F32;
    use crate::linear_algebra::vec3;
    use crate::linear_algebra::vector::types::{Vector2F32, Vector3, Vector3F32};
    use crate::physics::collider::Collider;
    use crate::physics::collider::sphere_collider::SphereCollider;
    use crate::physics::rigidbody::Rigidbody;
    use crate::physics::rigidbody_handle::RigidbodyHandle;
    use crate::physics::world::World;
    use crate::shared::angle::Angle::Degrees;
    use crate::shared::traits::lerp::Lerp;

    #[test]
    fn test_vec_funcs() {
        let a = vec3(1_f32, 2_f32, 3_f32);
        let b = vec3(4_f32, 5_f32, 6_f32);

        let dot = a.dot(&b);
        let c = a.cross(&b);

        println!("{} {}", dot, c);
    }

    #[test]
    fn test_uv_sphere() {
        let sphere = UVSphere {
            radius: 5.0,
            latitudes: 10.0,
            longitudes: 10.0,
        };

        let mut rgb = image::DynamicImage::new(100, 100, ColorType::Rgb8);

        for v in sphere.to_geometry().vertices {
            rgb.put_pixel(50 + v.x() as u32, 50 + v.y() as u32, Rgba([255; 4]))
        }

        rgb.save("output.png").unwrap();
        //rgb.put_pixel(50 + )
    }

    #[test]
    fn test_matrix() {
        let a = Mat4F32::look_at(
            Vector3F32::BACKWARD * 3_f32,
            Vector3F32::ZERO,
            Vector3F32::UP,
        );

        let b = nalgebra_glm::look_at(
            &nalgebra_glm::Vec3::new(0_f32, 0_f32, -3.0),
            &nalgebra_glm::Vec3::new(0_f32, 0_f32, 0.0),
            &nalgebra_glm::Vec3::new(0_f32, 1_f32, 0_f32),
        );

        println!("{}", a);
        println!("{}", b);
    }

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

    #[test]
    fn test_world() {
        let mut world = World::default();
        let handle = world.create_rigidbody();

        let mut now = Instant::now();

        while now.elapsed().as_secs() < 5 {
            world.update();
        }

        println!("{:?}", handle.rigidbody);
    }

    #[test]
    fn test_ffmpeg() {
        let dims = [512; 2];
        let mut pos = [0; 2];

        let mut magnitude = 30;

        std::fs::create_dir("tmp");

        //let mut img = image::DynamicImage::new(dims[0], dims[1], ColorType::Rgb8).to_rgb8();

        let mut threads = vec![];

        for j in 0..10 {
            let handle = spawn(move || {
                let mut angle = 0_f32;

                let mut img = image::DynamicImage::new(dims[0], dims[1], ColorType::Rgb8).to_rgb8();

                magnitude = (angle * 256_f32) as i32;

                let mut last_pos = pos;

                for i in (j * 100)..((j + 1) * 100) {
                    angle = i as f32 / 1000_f32;
                    magnitude = (angle * 256_f32) as i32;
                    pos = [((angle * 360_f32).to_radians().cos() * magnitude as f32) as i32,
                        ((angle * 360_f32).to_radians().sin() * magnitude as f32) as i32];
                    img.put_pixel(
                        (256 + pos[0]) as u32,
                        (256 + pos[1]) as u32,
                        Rgb([(angle * 255_f32) as u8, 255, 0]),
                    );
                    img.save(format!("tmp/tmp_{i}.png")).unwrap();
                    last_pos = pos;
                }
            });
            threads.push(handle);
        }

        while !threads.is_empty() {
            let t = threads.remove(0);
            t.join();
        }

        /*for i in 0..1000 {
            angle = i as f32 / 1000_f32;
            magnitude = (angle * 256_f32) as i32;
            pos = [((angle * 360_f32).to_radians().cos() * magnitude as f32) as i32,
                ((angle * 360_f32).to_radians().sin() * magnitude as f32) as i32];

            img.put_pixel(
                (256 + pos[0]) as u32,
                (256 + pos[1]) as u32,
                Rgb([(angle * 255_f32) as u8, 255, 0]),
            );
            img.save(format!("tmp/tmp_{i}.png")).unwrap();
            //pos[1] += 1;
            //magnitude += 1;
            last_pos = pos;
        }*/

        let mut cmd =
            Command::new("ffmpeg.exe")
                .arg("-framerate")
                .arg("120")
                .arg("-i")
                .arg("tmp/tmp_%01d.png")
                .arg("out.mp4")
                .output();
        let out = cmd.unwrap().stdout;
    }

    #[test]
    fn test_world2() {
        let mut world = World::default();
        let mut instant = Instant::now();
        let rigid = world.register_rigidbody(Rigidbody::default());

        while instant.elapsed().as_secs() < 5 {
            world.update();
        }

        let mut sphere_collider = SphereCollider::new(5_f64);
        sphere_collider.set_handle(RigidbodyHandle {
            id: 0,
            rigidbody: Default::default(),
        });

        let mut sphere_collider_2 = SphereCollider::new(5_f64);
        sphere_collider_2.set_handle(RigidbodyHandle {
            id: 0,
            rigidbody: {
                let r = Rigidbody::default();
                r.set_position(vec3(7.0, 0.0, 0.0));
                r
            },
        });

        let col = sphere_collider.check_collision(&sphere_collider_2);
        println!("{:?}", col);
    }

    #[test]
    fn test_summation() {
        println!("{}", summation(1, 10, |i| 2_f32.powf(i as f32)));
    }

    #[test]
    fn test_ref() {
        let mut r = Matrix::new([
            [2_f32, 4_f32, 6_f32, 8_f32],
            [0_f32, 1_f32, 0_f32, 0_f32],
            [0_f32, 0_f32, 3_f32, 6_f32],
        ]);

    }

    #[test]
    fn test_ref2() {
        let mut r = Matrix::new([
            [1_f32, 2_f32, -3_f32],
            [4_f32, -5_f32, 6_f32]
        ]);

        println!("{}", r);
        r.mul_add_row(1, 0, 4_f32);
        println!("{}", r);
    }
}