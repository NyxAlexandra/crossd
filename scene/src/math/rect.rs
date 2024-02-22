use std::ops::{Add, Sub};

use super::{Point2, Rect, Size2};
use crate::{Path, Zero};

impl<T> Rect<T> {
    pub const fn new(point: Point2<T>, size: Size2<T>) -> Self {
        Self { point, size }
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Rect<U> {
        let Self { point, size } = self;

        let point = point.map(&mut f);
        let size = size.map(&mut f);

        Rect { point, size }
    }
}

impl<T> From<Rect<T>> for Path<T>
where
    T: Zero + Add<Output = T> + Sub<Output = T>,
{
    fn from(Rect { point, size }: Rect<T>) -> Self {
        let ne = point + (size.w, T::ZERO);
        let se = ne - (T::ZERO, size.h);
        let sw = se - (size.w, T::ZERO);

        let mut path = Path::builder();

        path.move_to(point);
        path.line_to(ne);
        path.line_to(se);
        path.line_to(sw);
        path.line_to(point);

        path.build().expect("rectangles are valid paths")
    }
}
