use bytemuck::{Pod, Zeroable};
use crossd_math::Zero;

use super::Point2;

impl<T> Point2<T> {
    /// A new point from x and y.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Point2<T> {
    /// Point at `(v, v)`.
    #[inline]
    #[must_use]
    pub const fn splat(v: T) -> Self {
        Self { x: v, y: v }
    }
}

impl<T: Zero> Point2<T> {
    /// Point `(0, 0)`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

unsafe impl<T: Pod> Pod for Point2<T> {}
unsafe impl<T: Zeroable> Zeroable for Point2<T> {}
