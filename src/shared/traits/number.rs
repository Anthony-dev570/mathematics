use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::mem::size_of;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

use crate::shared::endian::Endian;
use crate::shared::traits::lerp::Lerp;

crate::number!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);

#[macro_export]
macro_rules! number {
    ($($num:ty),*) => {
        $(
            impl Lerp for $num {
                fn lerp(&self, b: &Self, t: f32) -> Self {
                    ((1_f64 - t as f64) * (*self as f64) + (t as f64) * *b as f64) as Self
                }
            }

            impl Number for $num {
                const ZERO: Self = 0.0 as Self;
                const ONE: Self = 1.0 as Self;
                const TWO: Self = 2.0 as Self;

                const PI: Self = std::f64::consts::PI as Self;

                const TYPE: &'static str = stringify!($num);

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

                fn arc_tan2(self, b: Self) -> Self {
                    (self as f64).atan2(b as f64) as Self
                }

                fn from_f64(f: f64) -> Self {
                    f as Self
                }

                fn absolute(self) -> Self {
                    (self as f64).abs() as Self
                }

                fn clamp(self, min: Self, max: Self) -> Self {
                    if self < min {
                        return min;
                    }
                    if self > max {
                        return max;
                    }
                    self
                }

                fn to_i32(self) -> i32 {
                    self as i32
                }

                fn to_usize(self) -> usize {
                    self as usize
                }
            }
        )*
    };
}

pub trait Number: Sized + Copy + Debug + Default + ToString + Lerp + Display + PartialEq +
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
AddAssign + SubAssign + MulAssign + DivAssign + Sum<Self> + PartialOrd + Rem<Output=Self> {
    ///The size of this number object in memory, in bytes.
    const SIZE: usize = size_of::<Self>();

    ///The zero value of this number.
    const ZERO: Self;

    ///The one value of this number.
    const ONE: Self;

    ///The two value of this number.
    const TWO: Self;

    ///The pi constant of this number;
    const PI: Self;

    const TYPE: &'static str;

    ///Convert this number to its little endian form.
    fn num_to_le_bytes(&self) -> [u8; Self::SIZE];

    ///Convert this number to its big endian form.
    fn num_to_be_bytes(&self) -> [u8; Self::SIZE];

    ///Convert little endian byte array to this number.
    fn num_from_le_bytes(bytes: [u8; Self::SIZE]) -> Self;

    ///Convert big endian byte array to this number.
    fn num_from_be_bytes(bytes: [u8; Self::SIZE]) -> Self;

    ///Dynamically convert this number to an array of endian bytes.
    fn num_to_bytes(&self, endian: Endian) -> [u8; Self::SIZE] {
        match endian {
            Endian::Little => self.num_to_le_bytes(),
            Endian::Big => self.num_to_be_bytes()
        }
    }

    ///Dynamically convert array of endian bytes to this number.
    fn num_from_bytes(bytes: [u8; Self::SIZE], endian: Endian) -> Self {
        match endian {
            Endian::Little => Self::num_from_le_bytes(bytes),
            Endian::Big => Self::num_from_be_bytes(bytes)
        }
    }

    ///Raise this number to a power 'b'.
    fn num_pow(self, b: Self) -> Self;

    ///Returns the square root of this number.
    fn num_sqrt(self) -> Self;

    ///Returns the bigger of these two numbers.
    fn num_max(&self, b: &Self) -> Self;

    ///Returns the smaller of these two numbers.
    fn num_min(&self, b: &Self) -> Self;

    ///Converts this number into radians.<br>WARNING!!! No unit test is done at this level.
    fn rad(self) -> Self;

    ///Converts this number into degrees.<br>WARNING!!! No unit test is done at this level.
    fn deg(self) -> Self;

    ///Returns the cosine of this number.
    fn cosine(self) -> Self;

    ///Returns the sine of this number.
    fn sine(self) -> Self;

    ///Returns the tangent of this number.
    fn tangent(self) -> Self;

    ///Returns a tuple containing the cosine and sine of this number.
    fn cos_sin(self) -> (Self, Self) {
        (self.cosine(), self.sine())
    }

    ///Returns the inverse tangent of two numbers.
    fn arc_tan2(self, b: Self) -> Self;

    ///Converts a f64(double) into this number.
    fn from_f64(f: f64) -> Self;

    ///Calculates the absolute value of this number.
    fn absolute(self) -> Self;

    ///Clamps this number between min and max.
    fn clamp(self, min: Self, max: Self) -> Self;

    fn to_i32(self) -> i32;
    fn to_usize(self) -> usize;
}