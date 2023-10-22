use std::ops::{Add, Sub};

use bytemuck::{Pod, Zeroable};
use crossd_math::Float;

use super::Size2;
use crate::math::{One, Zero};

impl<T> Size2<T> {
    #[must_use]
    #[inline(always)]
    pub const fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}

impl<T: Copy> Size2<T> {
    #[must_use]
    #[inline(always)]
    pub const fn splat(v: T) -> Self {
        Self { w: v, h: v }
    }
}

impl<T> Size2<T>
where
    T: Add<Output = T>,
{
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
}

impl<T> Size2<T>
where
    T: Sub<Output = T>,
{
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

impl<T: Zero> Size2<T> {
    /// A size with width and height set to `T::ZERO`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: One> Size2<T> {
    /// A size with width and height set to `T::ONE`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<T: Float> Size2<T> {
    /// Round to the nearest integer.
    pub fn snap(self) -> Self {
        Self::new(self.w.snap(), self.h.snap())
    }

    /// Round to the nearest integer and convert to an integer.
    pub fn round(self) -> Size2<T::Int> {
        Size2::new(self.w.round(), self.h.round())
    }
}

impl<T> Add for Size2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.extend(rhs)
    }
}

impl<T> Sub for Size2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.reduce(rhs)
    }
}

unsafe impl<T: Pod> Pod for Size2<T> {}
unsafe impl<T: Zeroable> Zeroable for Size2<T> {}
