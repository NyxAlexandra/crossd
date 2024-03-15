use super::{Point2, Triangle};
use crate::scene::Path;

impl<T> Triangle<T> {
    pub fn new(
        a: impl Into<Point2<T>>,
        b: impl Into<Point2<T>>,
        c: impl Into<Point2<T>>,
    ) -> Self {
        Self(a.into(), b.into(), c.into())
    }
}

impl From<Triangle<f32>> for Path {
    fn from(Triangle(a, b, c): Triangle<f32>) -> Self {
        let mut builder = Path::builder();

        builder.move_to(a);
        builder.line_to(b);
        builder.line_to(c);

        builder.build().expect("all triangles produce valid paths")
    }
}
