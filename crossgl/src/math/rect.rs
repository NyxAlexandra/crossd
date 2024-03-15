use super::{Point2, Rect, Size2};
use crate::scene::Path;

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

impl From<Rect<f32>> for Path {
    fn from(Rect { point, size }: Rect<f32>) -> Self {
        let ne = point + (size.w, 0.0);
        let se = ne - (0.0, size.h);
        let sw = se - (size.w, 0.0);

        let mut path = Path::builder();

        path.move_to(point);
        path.line_to(ne);
        path.line_to(se);
        path.line_to(sw);
        path.line_to(point);

        path.build().expect("rectangles are valid paths")
    }
}

impl<T> FromIterator<Point2<T>> for Rect<T> {
    fn from_iter<I: IntoIterator<Item = Point2<T>>>(iter: I) -> Self {
        todo!()
    }
}

impl<'a, T> FromIterator<&'a Point2<T>> for Rect<T> {
    fn from_iter<I: IntoIterator<Item = &'a Point2<T>>>(iter: I) -> Self {
        todo!()
    }
}
