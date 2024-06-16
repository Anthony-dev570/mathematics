use crate::geometry::shape::Shape;
use crate::geometry::triangle::triangle2d::Triangle2D;
use crate::shared::traits::number::Number;

impl <N: Number> Shape<N> for Triangle2D<N> {
    fn area(&self) -> N {
        match self {
            Triangle2D::FromVertices { a, b, c } => {
                let (x1, y1) = (a[0], a[1]);
                let (x2, y2) = (b[0], b[1]);
                let (x3, y3) = (c[0], c[1]);
                (N::ONE / N::TWO) * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).absolute()
            }
        }
    }
}