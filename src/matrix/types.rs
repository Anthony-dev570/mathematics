use crate::matrix::Matrix;
use crate::number::Number;

pub type SquareMatrix<const L: usize, N: Number> = Matrix<L, L, N>;

crate::make_square_matrices! {
    { 2, 3, 4, 5, 6, 7, 8, 9 }
}

#[macro_export]
macro_rules! to_mat {
    (
        $N:ident, $dimension:literal => $($mat_convert:literal),*
    ) => {
        paste::paste! {
            $(
                pub fn [<to_mat $mat_convert>](self) -> crate::matrix::types::[<Mat $mat_convert>]<N> {
                    let mut out = crate::matrix::types::[<Mat $mat_convert>]::identity();

                    for i in 0..$dimension.min($mat_convert) {
                        for j in 0..$dimension.min($mat_convert) {
                            out.0[i][j] = self.0[i][j];
                        }
                    }
                    //let

                    out
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! make_square_matrices {
    (
        {$($matrix_size:literal),*}
    ) => {
        paste::paste! {
            $(
                pub type [<Mat $matrix_size>]<N: Number> = SquareMatrix<$matrix_size, N>;

                pub type [<Mat $matrix_size i8>] = [<Mat $matrix_size>]<i8>;
                pub type [<Mat $matrix_size i16>] = [<Mat $matrix_size>]<i16>;
                pub type [<Mat $matrix_size i32>] = [<Mat $matrix_size>]<i32>;
                pub type [<Mat $matrix_size i64>] = [<Mat $matrix_size>]<i64>;
                pub type [<Mat $matrix_size i128>] = [<Mat $matrix_size>]<i128>;

                pub type [<Mat $matrix_size u8>] = [<Mat $matrix_size>]<u8>;
                pub type [<Mat $matrix_size u16>] = [<Mat $matrix_size>]<u16>;
                pub type [<Mat $matrix_size u32>] = [<Mat $matrix_size>]<u32>;
                pub type [<Mat $matrix_size u64>] = [<Mat $matrix_size>]<u64>;
                pub type [<Mat $matrix_size u128>] = [<Mat $matrix_size>]<u128>;

                pub type [<Mat $matrix_size F32>] = [<Mat $matrix_size>]<f32>;
                pub type [<Mat $matrix_size F64>] = [<Mat $matrix_size>]<f64>;
            )*
        }
    };
}