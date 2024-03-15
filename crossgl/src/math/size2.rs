use super::Size2;
use crate::math::{Point2, Rect};
use crate::num::Zero;

impl<T> Size2<T> {
    pub const fn new(w: T, h: T) -> Self {
        Self { w, h }
    }

    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Size2<U> {
        Size2::new(f(self.w), f(self.h))
    }
}

impl<T: Zero> Size2<T> {
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<T> From<Rect<T>> for Size2<T> {
    fn from(Rect { size, .. }: Rect<T>) -> Self {
        size
    }
}

impl<T: Zero> From<Size2<T>> for Rect<T> {
    fn from(size: Size2<T>) -> Self {
        Rect::new(Point2::ZERO, size)
    }
}
