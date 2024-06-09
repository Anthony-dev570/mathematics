#[macro_export]
macro_rules! vector {
    (
        $($dimension:literal),*
    ) => {
        paste::paste! {
            $(
                pub type [<Vector $dimension>]<N: crate::linear_algebra::vector::Number> = super::Vector<$dimension, N>;

                pub type [<Vector $dimension F32>] = super::Vector<$dimension, f32>;
                pub type [<Vector $dimension F64>] = super::Vector<$dimension, f64>;

                pub type [<Vector $dimension I8>] = super::Vector<$dimension, i8>;
                pub type [<Vector $dimension I16>] = super::Vector<$dimension, i16>;
                pub type [<Vector $dimension I32>] = super::Vector<$dimension, i32>;
                pub type [<Vector $dimension I64>] = super::Vector<$dimension, i64>;
                pub type [<Vector $dimension I128>] = super::Vector<$dimension, i128>;
                pub type [<Vector $dimension ISize>] = super::Vector<$dimension, isize>;

                pub type [<Vector $dimension U8>] = super::Vector<$dimension, u8>;
                pub type [<Vector $dimension U16>] = super::Vector<$dimension, u16>;
                pub type [<Vector $dimension U32>] = super::Vector<$dimension, u32>;
                pub type [<Vector $dimension U64>] = super::Vector<$dimension, u64>;
                pub type [<Vector $dimension U128>] = super::Vector<$dimension, u128>;
                pub type [<Vector $dimension USize>] = super::Vector<$dimension, usize>;
            )*
        }
    };
}

crate::vector! {
    2, 3, 4, 5, 6, 7, 8, 9
}