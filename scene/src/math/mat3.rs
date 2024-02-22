use crate::{Mat3, One, Vec2, Vec3, Zero};

impl<T> Mat3<T> {
    /// Create a matrix from it's columns.
    pub const fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: Vec3<T>) -> Self
    where
        T: Copy,
    {
        Self::new(v, v, v)
    }

    /// Create a matrix that can traslate [`Point2`]s and [`Vec2`]s.
    ///
    /// ```
    /// # use crossd_scene::{Vec2, Point2, Mat3};
    /// #
    /// let point = Point2::new(1, 1);
    /// let trans = Mat3::from_translation(Vec2::new(1, 1));
    ///
    /// assert_eq!(point * trans, Point2::new(2, 2));
    /// ```
    pub fn from_translation(v: Vec2<T>) -> Self {
        todo!()
    }

    pub fn transpose(self) -> Self {
        // 4  5  6
        // 7  8  9
        // 10 11 12

        // 4  7 10
        // 5  8 11
        // 6  9 12

        todo!()
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Mat3<U> {
        let Self { x, y, z } = self;

        let x = x.map(&mut f);
        let y = y.map(&mut f);
        let z = z.map(&mut f);

        Mat3 { x, y, z }
    }

    pub fn cast<U>(self) -> Mat3<U>
    where
        T: Into<U>,
    {
        Mat3::new(self.x.cast(), self.y.cast(), self.z.cast())
    }
}

impl<T: Zero> Mat3<T> {
    pub const ZERO: Self = Self::splat(Vec3::splat(T::ZERO));
}

impl<T: One> Mat3<T> {
    pub const ONE: Self = Self::splat(Vec3::splat(T::ONE));
}

impl<T: One + Zero> Mat3<T> {
    pub const IDENTITY: Self = Self::new(
        Vec3::new(T::ONE, T::ZERO, T::ZERO),
        Vec3::new(T::ZERO, T::ONE, T::ZERO),
        Vec3::new(T::ZERO, T::ZERO, T::ONE),
    );
}
