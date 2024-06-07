use std::fmt::Display;
use std::ops::{Index, IndexMut};
use crate::number::Number;

pub mod imp;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vector<const L: usize, N: Number>([N; L]);