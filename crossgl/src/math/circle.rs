use super::{Circle, Point2};
use crate::scene::Path;

impl<T> Circle<T> {
    pub fn new(point: impl Into<Point2<T>>, radius: T) -> Self {
        Self { point: point.into(), radius }
    }
}

impl<T> From<Circle<T>> for Path {
    fn from(Circle { point, radius }: Circle<T>) -> Self {
        todo!()
    }
}
