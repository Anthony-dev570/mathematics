use std::ops::Neg;

use crate::shared::traits::number::Number;

pub fn quadratic_formula<N: Number>(a: N, b: N, c: N) -> [N; 2] where N: Neg<Output=N> {
    let neg_b = -b;

    let numerator = (b.num_pow(N::TWO) - N::from_f64(4.0) * a * c).num_sqrt();
    let denominator = N::TWO * a;
    [
        (neg_b + numerator) / denominator,
        (neg_b - numerator) / denominator
    ]
}

pub fn slope_xx_yy<N: Number>(x1: N, y1: N, x2: N, y2: N) -> N {
    slope_deltax_deltay(x2 - x1, y2 - y1)
}

pub fn slope_deltax_deltay<N: Number>(delta_x: N, delta_y: N) -> N {
    delta_y / delta_x
}