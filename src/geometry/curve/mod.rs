use crate::linear_algebra::vector::types::Vector2;
use crate::shared::traits::lerp::Lerp;
use crate::shared::traits::number::Number;

///Represents a Bézier curve.
///See: <a href="https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Specific_cases">Bézier curve</a>
#[derive(Debug, Clone)]
pub enum Curve<N: Number> {
    Linear {
        p0: Vector2<N>,
        p1: Vector2<N>,
    },
    Quadratic {
        p0: Vector2<N>,
        p1: Vector2<N>,
        p2: Vector2<N>,
    },
    Cubic {
        p0: Vector2<N>,
        p1: Vector2<N>,
        p2: Vector2<N>,
        p3: Vector2<N>,
    },
}

impl<N: Number> Curve<N> {
    pub fn points(&self, precision: N) -> Vec<Vector2<N>> {
        let precision = precision.clamp(N::ZERO, N::ONE);

        let mut output = vec![];

        let mut sum = N::ZERO;

        while sum <= N::from_f64(0.99) {
            output.push(self.interpolate(sum));
            sum += precision;
        }
        output.push(*self.last());
        output
    }
    pub fn interpolate(&self, t: N) -> Vector2<N> {
        if t <= N::ZERO {
            return *self.first();
        } else if t >= N::ONE {
            return *self.last();
        }
        let inv_t = N::ONE - t;
        match self {
            Curve::Linear { p0, p1 } => {
                let (p0_x, p0_y) = p0.xy();
                let (p1_x, p1_y) = p1.xy();

                let pf_x = inv_t * p0_x + t * p1_x;
                let pf_y = inv_t * p0_y * t * p1_y;

                Vector2::new([pf_x, pf_y])
            }
            Curve::Quadratic { p0, p1, p2 } => {
                let (p0_x, p0_y) = p0.xy();
                let (p1_x, p1_y) = p1.xy();
                let (p2_x, p2_y) = p2.xy();

                let pf_x =
                    (N::ONE - t) * ((N::ONE - t) * p0_x + t * p1_x) + t * (inv_t * p1_x + t * p2_x);
                let pf_y =
                    (N::ONE - t) * ((N::ONE - t) * p0_y + t * p1_y) + t * (inv_t * p1_y + t * p2_y);

                Vector2::new([pf_x, pf_y])
            }
            Curve::Cubic { p0, p1, p2, p3 } => {
                let (p0_x, p0_y) = p0.xy();
                let (p1_x, p1_y) = p1.xy();
                let (p2_x, p2_y) = p2.xy();
                let (p3_x, p3_y) = p3.xy();

                let inv_t2 = inv_t * inv_t;
                let inv_t3 = inv_t2 * inv_t;
                let three = N::from_f64(3.0);

                let pf_x =
                    ((N::ONE - t) * (N::ONE - t) * (N::ONE - t)) * p0_x + (three * ((N::ONE - t) * ((N::ONE - t)))) * (t * p1_x) + three * ((N::ONE - t) * (t * t)) * p2_x + (t * t * t) * p3_x;

                let pf_y = ((N::ONE - t) * (N::ONE - t) * (N::ONE - t)) * p0_y + (three * ((N::ONE - t) * ((N::ONE - t)))) * (t * p1_y) + three * ((N::ONE - t) * (t * t)) * p2_y + (t * t * t) * p3_y;

                Vector2::new([pf_x, pf_y])
            }
        }
    }

    pub fn first(&self) -> &Vector2<N> {
        match self {
            Curve::Linear { p0, .. } => p0,
            Curve::Quadratic { p0, .. } => p0,
            Curve::Cubic { p0, .. } => p0
        }
    }

    pub fn last(&self) -> &Vector2<N> {
        match self {
            Curve::Linear { p1, .. } => p1,
            Curve::Quadratic { p2, .. } => p2,
            Curve::Cubic { p3, .. } => p3
        }
    }
}