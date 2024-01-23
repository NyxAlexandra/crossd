use std::mem::ManuallyDrop;
use std::ops::{Add, Div, Sub};

use bytemuck::{Pod, Zeroable};

use super::Point2;
use crate::{Num, Size2, Vec2, Zero};

union Cast<T> {
    point2: ManuallyDrop<Point2<T>>,
    vec2: ManuallyDrop<Vec2<T>>,
}

impl<T> Cast<T> {
    const fn point2(point2: Point2<T>) -> Self {
        Self { point2: ManuallyDrop::new(point2) }
    }

    const fn vec2(vec2: Vec2<T>) -> Self {
        Self { vec2: ManuallyDrop::new(vec2) }
    }
}

impl<T> Point2<T> {
    /// A new point from x and y.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn from_vec2(vec2: Vec2<T>) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast::vec2(vec2).point2 })
    }

    #[must_use]
    pub const fn to_vec2(self) -> Vec2<T> {
        ManuallyDrop::into_inner(unsafe { Cast::point2(self).vec2 })
    }

    pub fn map<U: Num>(self, f: impl Fn(T) -> U) -> Point2<U> {
        Point2::new(f(self.x), f(self.y))
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

impl<T: Num> Point2<T> {
    #[must_use]
    pub fn add_vec2(&self, vec2: Vec2<T>) -> Self {
        Self::new(self.x + vec2.x, self.y + vec2.y)
    }
}

impl<T: Num> Add<Vec2<T>> for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num> Sub<Vec2<T>> for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Num> Add<Size2<T>> for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Size2<T>) -> Self::Output {
        Self::new(self.x + rhs.w, self.y + rhs.h)
    }
}

impl<T: Num> Sub<Size2<T>> for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Size2<T>) -> Self::Output {
        Self::new(self.x - rhs.w, self.y - rhs.h)
    }
}

impl<T: Num> Div<T> for Point2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

unsafe impl<T: Pod> Pod for Point2<T> {}
unsafe impl<T: Zeroable> Zeroable for Point2<T> {}
