use crate::shared::traits::number::Number;

pub mod row2;
pub mod row3;
pub mod row4;
pub mod row5;
pub mod row6;
pub mod row7;
pub mod row8;
pub mod row9;

#[macro_export]
macro_rules! row {
    (
        $size:tt {
            $(
                $field:ident
            ),*
        }
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy)]
            pub enum [<Row $size>]<N: crate::linear_algebra::Number> {
                Raw {
                    $($field: N),*
                },
                Row([N; $size]),
                Vector(crate::linear_algebra::[<Vector $size>]<N>)
            }

            impl <N: crate::linear_algebra::Number> crate::linear_algebra::matrix::row::Row<$size, N> for [<Row $size>]<N> {
                fn to_array(self) -> [N; $size] {
                    match self {
                        Self::Raw { $($field),* } => [$($field),*],
                        Self::Row(row) => row,
                        Self::Vector(v) => v.0
                    }
                }
            }
        }
    };
}

pub trait Row<const R: usize, N: Number> {
    fn to_array(self) -> [N; R];
}