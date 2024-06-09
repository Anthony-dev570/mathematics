use crate::chemistry::temperature::Temperature;

impl Temperature {

    pub const fn from_celsius(celsius: f64) -> Temperature {
        Self {
            celsius,
            kelvin: Self::c_to_k(celsius),
            fahrenheit: Self::c_to_f(celsius),
        }
    }

    pub const fn from_kelvin(kelvin: f64) -> Temperature {
        Self {
            celsius: Self::k_to_c(kelvin),
            kelvin,
            fahrenheit: Self::k_to_f(kelvin),
        }
    }

    pub const fn from_fahrenheit(fahrenheit: f64) -> Temperature {
        Self {
            celsius: Self::f_to_c(fahrenheit),
            kelvin: Self::f_to_k(fahrenheit),
            fahrenheit,
        }
    }

    pub const fn c_to_k(c: f64) -> f64 {
        c + 273.15
    }

    pub const fn c_to_f(c: f64) -> f64 {
        (c * 1.8) + 32.0
    }

    pub const fn k_to_c(k: f64) -> f64 {
        k - 273.15
    }

    pub const fn k_to_f(k: f64) -> f64 {
        (k - 273.15) * 1.8 + 32.0
    }

    pub const fn f_to_c(f: f64) -> f64 {
        (f - 32.0) / 1.8
    }

    pub const fn f_to_k(f: f64) -> f64 {
        Self::c_to_k(Self::f_to_c(f))
    }

    pub fn celsius(&self) -> f64 {
        self.celsius
    }
    pub fn kelvin(&self) -> f64 {
        self.kelvin
    }
    pub fn fahrenheit(&self) -> f64 {
        self.fahrenheit
    }
}