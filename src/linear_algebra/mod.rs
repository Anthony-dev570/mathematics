use std::ops::Neg;

use crate::linear_algebra::matrix::Matrix;
use crate::linear_algebra::matrix::row::Row;
use crate::linear_algebra::vector::types::{Vector2, Vector3, Vector4, Vector5, Vector6, Vector7, Vector8, Vector9};
use crate::shared::traits::number::Number;

pub mod vector;
pub mod matrix;
pub mod quaternion;
pub mod euler_angles;
pub mod scalar;

#[macro_export]
macro_rules! mat {
    (
       $(
            $size:literal {
                $(
                    $field:ident
                ),*
            }
       ),*
    ) => {
        paste::paste! {
            $(
                pub fn [<mat $size>]<N: Number>(
                    $($field: crate::linear_algebra::matrix::row::[<row $size>]::[<Row $size>]<N>),*
                ) -> Matrix<$size, $size, N> {
                    Matrix([
                        $($field.to_array()),*
                    ])
                }
            )*
        }
    };
}
macro_rules! vec {
    (
         $(
            $size:literal {
                $(
                    $field:ident
                ),*
            }
       ),*
    ) => {
        paste::paste! {
            $(
                pub fn [<vec $size>]<N: Number>($($field: N),*) -> [<Vector $size>]<N> {
                    [<Vector $size>]::new([$($field),*])
                }
            )*
        }
    };
}

vec!(
    2 { x, y },
    3 { x, y, z },
    4 { x, y, z, w },
    5 { x, y, z, w, a },
    6 { x, y, z, w, a, b },
    7 { x, y, z, w, a, b, c },
    8 { x, y, z, w, a, b, c, d },
    9 { x, y, z, w, a, b, c, d, e }
);

mat!(
    2 { a, b },
    3 { a, b, c },
    4 { a, b, c, d },
    5 { a, b, c, d, e },
    6 { a, b, c, d, e, f },
    7 { a, b, c, d, e, f, g },
    8 { a, b, c, d, e, f, g, i },
    9 { a, b, c, d, e, f, g, i, j }
);