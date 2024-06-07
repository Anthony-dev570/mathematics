use crate::number::Number;

pub mod imp;
pub mod types;
pub mod square_matrix;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod mat5;
pub mod mat6;
pub mod mat7;
pub mod mat8;
pub mod mat9;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const C: usize, const R: usize, N: Number>([[N; C]; R]);