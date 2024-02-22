use crate::{Circle, Path, Point2};

impl<T> Circle<T> {
    pub const fn new(point: Point2<T>, radius: T) -> Self {
        Self { point, radius }
    }
}

impl<T> From<Circle<T>> for Path {
    fn from(Circle { point, radius }: Circle<T>) -> Self {
        todo!()
    }
}
