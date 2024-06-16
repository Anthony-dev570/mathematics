use crate::color::Color;
use crate::shared::angle::Angle;

#[derive(Eq, PartialEq)]
enum ColorSelect {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn to_rgb(self) -> Self {
        match self {
            Color::RGB { .. } => self,
            Color::RGBA { red, green, blue, .. } => {
                Self::RGB { red, green, blue }
            }
            Color::HSL { hue, saturation, lightness } => {
                let hue = hue.to_degrees().take();
                let saturation = saturation / 100_f32;
                let lightness = lightness / 100_f32;

                let c = (1_f32 - (2_f32 * lightness - 1_f32).abs()) * saturation;

                let x = c * (1_f32 - ((hue / 60_f32) % 2_f32 - 1_f32).abs());

                let m = lightness - c / 2_f32;

                let sixty = 60_f32;

                let options = [
                    (c, x, 0_f32),
                    (x, c, 0_f32),
                    (0_f32, c, x),
                    (0_f32, x, c),
                    (x, 0_f32, c),
                    (c, 0_f32, x)
                ];

                let mut angle = 0_f32;

                for i in 0..options.len() {
                    let a = angle;
                    let b = a + sixty;

                    if a <= hue && hue < b {
                        let values = options[i];
                        return Self::RGB {
                            red: values.0 + m,
                            green: values.1 + m,
                            blue: values.2 + m,
                        };
                    }

                    angle = b;
                }

                panic!("Not possible.")
            }
            Color::HSLA { hue, saturation, lightness, .. } => {
                Self::HSL { hue, saturation, lightness }.to_rgb()
            }
            Color::HSV { hue, saturation, value } => {
                let hue = hue.to_degrees().take();
                let saturation = saturation / 100_f32;
                let value = value / 100_f32;

                let c = value * saturation;

                let x = c * (1_f32 - (hue / 60_f32) % 2_f32 - 1_f32);

                let m = value - c;

                let z = 0_f32;

                let values = [
                    (c, x, z),
                    (x, c, z),
                    (z, c, x),
                    (z, c, x),
                    (z, x, c),
                    (x, z, c),
                    (c, z, x)
                ];

                let mut angle = 0_f32;

                for i in 0..values.len() {
                    let a = angle;
                    let b = angle + 60_f32;

                    if a <= hue && hue < b {
                        let color = values[i];
                        return Color::RGB {
                            red: color.0 + m,
                            green: color.1 + m,
                            blue: color.2 + m,
                        };
                    }
                }
                panic!("Not possible")
            }
            Color::HSVA { hue, saturation, value, .. } => {
                Self::HSV { hue, saturation, value }.to_rgb()
            }
        }
    }
    pub fn to_rgba(self) -> Self {
        match self {
            Color::RGB { red, green, blue } => {
                return Self::RGBA { red, green, blue, alpha: 1_f32 };
            }
            Color::RGBA { .. } => self,
            Color::HSL { .. } => {
                if let Self::RGB { red, green, blue } = self.to_rgb() {
                    return Self::RGBA {
                        red,
                        green,
                        blue,
                        alpha: 1.0,
                    };
                }
                panic!("Not possible")
            }
            Color::HSLA { alpha, .. } => {
                if let Self::RGB { red, green, blue } = self.to_rgb() {
                    return Self::RGBA {
                        red,
                        green,
                        blue,
                        alpha,
                    };
                }
                panic!("Not possible")
            }
            Color::HSV { .. } => {
                if let Self::RGB { red, green, blue } = self.to_rgb() {
                    return Self::RGBA {
                        red,
                        green,
                        blue,
                        alpha: 1.0,
                    };
                }
                panic!("Not possible")
            }
            Color::HSVA { alpha, .. } => {
                if let Self::RGB { red, green, blue } = self.to_rgb() {
                    return Self::RGBA {
                        red,
                        green,
                        blue,
                        alpha,
                    };
                }
                panic!("Not possible")
            }
        }
    }

    pub fn to_hsl(self) -> Self {
        match self {
            Color::RGB { red, green, blue } => {
                let (mut color, mut c_max, mut c_min) = (ColorSelect::Red, red, red);

                if green > c_max {
                    c_max = green;
                    color = ColorSelect::Green;
                }
                if blue > c_max {
                    c_max = blue;
                    color = ColorSelect::Blue;
                }

                if green < c_min {
                    c_min = green;
                    color = ColorSelect::Green;
                }

                if blue < c_min {
                    c_min = blue;
                    color = ColorSelect::Blue;
                }

                let delta = c_max - c_min;

                let h = {
                    let mut h = 0_f32;

                    if delta != 0_f32 {
                        h = 60_f32 * match color {
                            ColorSelect::Red => ((green - blue) / delta) % 6_f32,
                            ColorSelect::Green => ((blue - red) / delta) + 2_f32,
                            ColorSelect::Blue => ((red - green) / delta) + 4_f32
                        };
                    }
                    h
                };

                let l = (c_max + c_min) / 2_f32;

                let s = match delta.abs() > 0_f32 {
                    true => delta / (1_f32 - (2_f32 * l - 1_f32).abs()),
                    false => 0_f32
                };

                Color::HSL {
                    hue: Angle::Degrees(h),
                    saturation: s,
                    lightness: l,
                }
            }
            Color::RGBA { red, green, blue, .. } => {
                Self::RGB {
                    red,
                    green,
                    blue,
                }.to_hsl()
            }
            Color::HSL { .. } => todo!(),
            Color::HSLA { .. } => todo!(),
            Color::HSV { .. } => todo!(),
            Color::HSVA { .. } => todo!(),
        }
    }
}