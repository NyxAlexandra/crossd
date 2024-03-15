use super::{Point2, Vec2};
use crate::num::{Float, Max, Min, NegOne, One, Zero};

impl<T> Vec2<T> {
    /// Construct a new vector from it's components.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Set all components of the vector to `v` via copying.
    ///
    /// ```
    /// assert_eq!(Vec2U::splat(0), Vec2U::new(0, 0));
    /// ```
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    /// Map the components of this vector.
    ///
    /// ```
    /// let vec2f = Vec2F::new(25.3, 33.7);
    /// let vec2u: Vec2U = vec2f.map(|f32| f32 as u32);
    /// ```
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Vec2<U> {
        Vec2::new(f(self.x), f(self.y))
    }

    /// Round to the nearest integer.
    ///
    /// ```
    /// let vec = Vec2F::new(0.7, 0.1);
    /// let rounded = vec.round();
    ///
    /// assert_eq!(rounded, Vec2::new(1.0, 0.0));
    /// ```
    pub fn round(self) -> Self
    where
        T: Float,
    {
        self.map(|float| float.round())
    }

    /// Round to the nearest integer and convert to an integer.
    ///
    /// ```
    /// let vec = Vec2F::new(0.7, 0.1);
    /// let snapped = vec.snap();
    ///
    /// assert_eq!(snapped, Vec2I::new(1, 0));
    /// ```
    pub fn snap(self) -> Vec2<T::Int>
    where
        T: Float,
    {
        self.map(|float| float.snap())
    }
}

impl<T: Zero> Vec2<T> {
    /// Vector with components set to zero.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: One> Vec2<T> {
    /// Vector with components set to one.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<T: NegOne> Vec2<T> {
    /// Vector with components set to negative one.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: Max> Vec2<T> {
    /// Vector with components set to the maximum number.
    pub const MAX: Self = Self::splat(T::MAX);
}

impl<T: Min> Vec2<T> {
    /// Vector with components set to the minimum number.
    pub const MIN: Self = Self::splat(T::MIN);
}

impl<T> From<Point2<T>> for Vec2<T> {
    fn from(Point2 { x, y }: Point2<T>) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(Vec2 { x, y }: Vec2<T>) -> Self {
        (x, y)
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(Vec2 { x, y }: Vec2<T>) -> Self {
        [x, y]
    }
}
