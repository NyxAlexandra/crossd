use bytemuck::{Pod, Zeroable};

use super::Point2;

impl<T> Point2<T> {
    /// A new point from x and y.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

unsafe impl<T: Pod> Pod for Point2<T> {}
unsafe impl<T: Zeroable> Zeroable for Point2<T> {}
