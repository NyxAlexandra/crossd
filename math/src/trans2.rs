use std::mem::ManuallyDrop;
use std::ops::Mul;

use bytemuck::{Pod, Zeroable};

use super::Trans2;
use crate::{Mat2, Mat3x2, Num, One, Point2, Vec2, Zero};

union Cast<T> {
    trans2: ManuallyDrop<Trans2<T>>,
    mat3x2: ManuallyDrop<Mat3x2<T>>,
}

impl<T> Cast<T> {
    const fn trans2(trans2: Trans2<T>) -> Self {
        Self { trans2: ManuallyDrop::new(trans2) }
    }

    const fn mat3x2(mat3x2: Mat3x2<T>) -> Self {
        Self { mat3x2: ManuallyDrop::new(mat3x2) }
    }
}

impl<T> Trans2<T> {
    /// A new translation matrix from it's matrix and translation components.
    #[inline]
    #[must_use]
    pub const fn new(mat2: Mat2<T>, trans: Vec2<T>) -> Self {
        Self { mat2, trans }
    }

    /// Convert from a [`Mat3x2`].
    #[inline]
    #[must_use]
    pub const fn from_mat3x2(mat3x2: Mat3x2<T>) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast::mat3x2(mat3x2).trans2 })
    }

    /// Convert to a [`Mat3x2`].
    #[inline]
    #[must_use]
    pub const fn to_mat3x2(self) -> Mat3x2<T> {
        ManuallyDrop::into_inner(unsafe { Cast::trans2(self).mat3x2 })
    }
}

impl<T: One + Zero> Trans2<T> {
    pub const IDENTITY: Self = Self::new(Mat2::IDENTITY, Vec2::ZERO);
}

impl<T: One + Zero> Default for Trans2<T> {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl<T: Num> Mul<Vec2<T>> for Trans2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self.mat2.mul_vec2(rhs)
    }
}

impl<T: Num> Mul<Point2<T>> for Trans2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: Point2<T>) -> Self::Output {
        self.mat2.mul_point2(rhs).add_vec2(self.trans)
    }
}

unsafe impl<T: Pod> Pod for Trans2<T> {}
unsafe impl<T: Zeroable> Zeroable for Trans2<T> {}
