use crate::geometry::shape::Shape;
use crate::linear_algebra::vector::types::Vector2;
use crate::linear_algebra::vector::Vector;
use crate::shared::traits::number::Number;

#[derive(Debug, Clone, Copy)]
pub enum Rect<N: Number> {
    Bounds {
        position: Vector2<N>,
        size: Vector2<N>,
    }
}

impl<N: Number> Rect<N> {
    pub fn length(&self) -> N {
        match self {
            Rect::Bounds { size, .. } => {
                size.y()
            }
        }
    }

    pub fn width(&self) -> N {
        match self {
            Rect::Bounds { size, .. } => {
                size.x()
            }
        }
    }
}

impl<N: Number> Shape<2, N> for Rect<N> {
    fn area(&self) -> N {
        self.length() * self.width()
    }

    fn contains_point(&self, point: &Vector<2, N>) -> bool {
        match self {
            Rect::Bounds { position, size } => {
                let b = *position + *size;

                point.x() >= position.x() && point.x() <= b.x()
                    &&
                point.y() >= position.y() && point.y() <= b.y()
            }
        }
    }
}