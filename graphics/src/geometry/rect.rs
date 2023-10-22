use bytemuck::{Pod, Zeroable};
use crossd_math::Zero;

use super::{Point2, Rect, Size2};

impl<T> Rect<T> {
    #[inline]
    #[must_use]
    pub const fn new(loc: Point2<T>, size: Size2<T>) -> Self {
        Self { loc, size }
    }
}

impl<T: Zero> Rect<T> {
    /// A rectangle at `(0, 0)` with the given size.
    #[inline]
    #[must_use]
    pub const fn with_size(size: Size2<T>) -> Self {
        Self::new(Point2::ZERO, size)
    }
}

unsafe impl<T: Pod> Pod for Rect<T> {}
unsafe impl<T: Zeroable> Zeroable for Rect<T> {}
