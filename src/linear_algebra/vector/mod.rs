use std::fmt::Display;
use std::ops::{Index, IndexMut};
use crate::shared::traits::number::Number;

pub mod imp;
pub mod types;
pub mod operations;

pub mod vec2;
pub mod vec3;
pub mod vec4;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
///A vector is an N-Dimensional structure of scalar(numbers), representing a 1xN column matrix.
pub struct Vector<const L: usize, N: Number>(pub [N; L]);