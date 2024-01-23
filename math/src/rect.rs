use std::ops::Div;

use bytemuck::{Pod, Zeroable};

use super::{Point2, Rect};
use crate::{Num, Size2};

impl<T: Num, U: Num> Rect<T, U> {
    /// A new rectangle.
    #[inline]
    #[must_use]
    pub const fn new(loc: Point2<T>, size: Size2<U>) -> Self {
        Self { loc, size }
    }
}

impl<T: Num> Rect<T> {
    pub fn center(self) -> Point2<T>
    where
        T: From<u8>,
    {
        self.loc + self.size / T::from(2)
    }

    pub fn map<U: Num>(self, f: impl Fn(T) -> U) -> Rect<U> {
        Rect::new(self.loc.map(&f), self.size.map(&f))
    }
}

impl<T: Num> Div<T> for Rect<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.loc / rhs, self.size / rhs)
    }
}

unsafe impl<T: Pod> Pod for Rect<T> {}
unsafe impl<T: Zeroable> Zeroable for Rect<T> {}
