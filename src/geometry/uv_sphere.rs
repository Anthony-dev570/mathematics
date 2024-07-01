use crate::geometry::Geometry;
use crate::linear_algebra::vector::types::{Vector2, Vector3};
use crate::shared::traits::number::Number;

#[derive(Debug, Clone, Copy)]
pub struct UVSphere<N: Number> {
    pub radius: N,
    pub latitudes: N,
    pub longitudes: N
}

impl <N: Number> UVSphere<N> {
    pub fn to_geometry(self) -> Geometry<3, N> {
        let longitudes = self.longitudes.num_min(&N::from_f64(3.0));
        let latitudes = self.latitudes.num_min(&N::TWO);

        let mut vertices = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut indices = vec![];

        let length_inv = N::ONE / self.radius;

        let delta_latitude = N::PI / latitudes;
        let delta_longitudes = N::TWO * N::PI / longitudes;
        let mut latitude_angle = N::ZERO;
        let mut longitude_angle = N::ZERO;

        for i in 0..latitudes.to_usize() {
            latitude_angle = N::PI / N::TWO - N::from_f64(i as f64) * delta_latitude;

            let xy = self.radius * latitude_angle.cosine();
            let z = self.radius * latitude_angle.sine();

            for j in 0..longitudes.to_usize() {
                longitude_angle = N::from_f64(j as f64) * delta_longitudes;

                let v = Vector3::new([
                    xy * longitude_angle.cosine(),
                    xy * longitude_angle.cosine(),
                    z
                ]);

                let uv = Vector2::new([
                    N::from_f64(j as f64) / longitudes,
                    N::from_f64(i as f64) / latitudes
                ]);
                let normal = v * length_inv;

                vertices.push(v);
                uvs.push(uv);
                normals.push(normal);
            }
        }

        let (mut k1, mut k2) = (0, 0);

        for i in 0..latitudes.to_usize() {
            //k1 = N::from_f64(i) * (longitudes + N::ONE);
            k1 = i as u32 * (longitudes.to_i32() as u32 + 1);
            k2 = k1 + longitudes.to_i32() as u32 + 1;

            for _ in 0..longitudes.to_usize() {
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }

                if i != (latitudes.to_usize() - 1) {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k2 + 1);
                }
            }
        }

        Geometry {
            vertices,
            normals: Some(normals),
            uv: uvs,
            indices,
        }
    }
}