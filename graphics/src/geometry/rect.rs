use bytemuck::{Pod, Zeroable};

use super::{Point2, Rect, Size2};

impl<T> Rect<T> {
    #[inline]
    #[must_use]
    pub const fn new(loc: Point2<T>, size: Size2<T>) -> Self {
        Self { loc, size }
    }
}

unsafe impl<T: Pod> Pod for Rect<T> {}
unsafe impl<T: Zeroable> Zeroable for Rect<T> {}
