use crate::shared::angle::Angle;
use crate::shared::traits::number::Number;

pub mod imp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    RGB {
        red: f32,
        green: f32,
        blue: f32
    },
    RGBA {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32
    },
    HSL {
        hue: Angle<f32>,
        saturation: f32,
        lightness: f32
    },
    HSLA {
        hue: Angle<f32>,
        saturation: f32,
        lightness: f32,
        alpha: f32
    },
    HSV {
        hue: Angle<f32>,
        saturation: f32,
        value: f32,
    },
    HSVA {
        hue: Angle<f32>,
        saturation: f32,
        value: f32,
        alpha: f32
    }
}