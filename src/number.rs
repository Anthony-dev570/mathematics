use std::fmt::Debug;
use std::iter::Sum;
use std::mem::size_of;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::endian::Endian;

crate::number!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);

#[macro_export]
macro_rules! number {
    ($($num:ty),*) => {
        $(
            impl Number for $num {
                const ZERO: Self = 0.0 as Self;
                const ONE: Self = 1.0 as Self;
                const TWO: Self = 2.0 as Self;

                fn num_to_le_bytes(&self) -> [u8; { Self::SIZE }] {
                    self.to_le_bytes()
                }
                fn num_to_be_bytes(&self) -> [u8; { Self::SIZE }] {
                    self.to_be_bytes()
                }

                fn num_from_le_bytes(bytes: [u8; Self::SIZE]) -> Self {
                    Self::from_le_bytes(bytes)
                }
                fn num_from_be_bytes(bytes: [u8; Self::SIZE]) -> Self {
                    Self::from_be_bytes(bytes)
                }

                fn num_pow(self, b: Self) -> Self {
                    (self as f64).powf(b as f64) as Self
                }

                fn num_sqrt(self) -> Self {
                    (self as f64).sqrt() as Self
                }

                fn num_max(&self, b: &Self) -> Self {
                    if *self > *b {
                        return *self;
                    }
                    *b
                }

                fn num_min(&self, b: &Self) -> Self {
                    if *self < *b {
                        return *self;
                    }
                    *b
                }
                fn rad(self) -> Self {
                    (self as f64).to_radians() as Self
                }
                fn deg(self) -> Self {
                    (self as f64).to_degrees() as Self
                }

                fn sine(self) -> Self {
                    (self as f64).sin() as Self
                }
                fn cosine(self) -> Self {
                    (self as f64).cos() as Self
                }
                fn tangent(self) -> Self {
                    (self as f64).tan() as Self
                }
            }
        )*
    };
}

pub trait Number: Sized + Copy + Debug + Default + ToString +
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
AddAssign + SubAssign + MulAssign + DivAssign + Sum<Self> + PartialOrd {
    const SIZE: usize = size_of::<Self>();
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    fn num_to_le_bytes(&self) -> [u8; Self::SIZE];
    fn num_to_be_bytes(&self) -> [u8; Self::SIZE];

    fn num_from_le_bytes(bytes: [u8; Self::SIZE]) -> Self;
    fn num_from_be_bytes(bytes: [u8; Self::SIZE]) -> Self;

    fn num_to_bytes(&self, endian: Endian) -> [u8; Self::SIZE] {
        match endian {
            Endian::Little => self.num_to_le_bytes(),
            Endian::Big => self.num_to_be_bytes()
        }
    }

    fn num_from_bytes(bytes: [u8; Self::SIZE], endian: Endian) -> Self {
        match endian {
            Endian::Little => Self::num_from_le_bytes(bytes),
            Endian::Big => Self::num_from_be_bytes(bytes)
        }
    }

    fn num_pow(self, b: Self) -> Self;
    fn num_sqrt(self) -> Self;

    fn num_max(&self, b: &Self) -> Self;
    fn num_min(&self, b: &Self) -> Self;

    fn rad(self) -> Self;
    fn deg(self) -> Self;

    fn cosine(self) -> Self;
    fn sine(self) -> Self;
    fn tangent(self) -> Self;
    fn cos_sin(self) -> (Self, Self) {
        (self.cosine(), self.sine())
    }
}