use std::ops::Div;

use bytemuck::{Pod, Zeroable};

use super::Size2;
use crate::{Num, One, Zero};

impl<T> Size2<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(w: T, h: T) -> Self {
        Self { w, h }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Size2<U> {
        Size2::new(f(self.w), f(self.h))
    }
}

impl<T: Num> Size2<T> {
    #[must_use]
    #[inline(always)]
    pub const fn splat(v: T) -> Self {
        Self { w: v, h: v }
    }

    #[inline]
    #[must_use]
    pub fn extend(self, size: Self) -> Self {
        Self::new(self.w + size.w, self.h + size.h)
    }

    #[inline]
    #[must_use]
    pub fn extend_width(self, w: T) -> Self {
        Self::new(self.w + w, self.h)
    }

    #[inline]
    #[must_use]
    pub fn extend_height(self, h: T) -> Self {
        Self::new(self.w, self.h + h)
    }

    #[inline]
    #[must_use]
    pub fn reduce(self, size: Self) -> Self {
        Self::new(self.w - size.w, self.h - size.h)
    }

    #[inline]
    #[must_use]
    pub fn reduce_width(self, w: T) -> Self {
        Self::new(self.w - w, self.h)
    }

    #[inline]
    #[must_use]
    pub fn reduce_height(self, h: T) -> Self {
        Self::new(self.w, self.h - h)
    }
}

impl<T: One> Size2<T> {
    /// A size with width and height set to `T::ONE`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<T: Zero> Size2<T> {
    /// A size with width and height set to `T::ZERO`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: Num> Div<T> for Size2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.w / rhs, self.h / rhs)
    }
}

unsafe impl<T: Pod> Pod for Size2<T> {}
unsafe impl<T: Zeroable> Zeroable for Size2<T> {}
