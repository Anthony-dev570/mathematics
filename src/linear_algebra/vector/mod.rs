use std::fmt::Display;
use std::ops::{Index, IndexMut};
use crate::shared::traits::number::Number;

pub mod imp;
pub mod types;
pub mod operations;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
///A vector is an N-Dimensional structure of scalar(numbers), representing a 1xN column matrix.
pub struct Vector<const L: usize, N: Number>([N; L]);