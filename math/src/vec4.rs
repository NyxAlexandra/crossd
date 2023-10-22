use std::fmt;
use std::mem::ManuallyDrop;
use std::ops::{
    Add,
    AddAssign,
    Deref,
    DerefMut,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
};

use bytemuck::{Pod, Zeroable};
use mint::IntoMint;

use super::{Max, Min, NegOne, Num, One, Vec4, Zero};

impl<T: fmt::Display> fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {} {} {} {} ]", self.x, self.y, self.z, self.w)
    }
}

/// A union for casting between different representations of a [`Vec4`].
#[repr(C)]
union Cast<T> {
    vec4: ManuallyDrop<Vec4<T>>,

    array: ManuallyDrop<[T; 4]>,
    tuple: ManuallyDrop<(T, T, T, T)>,
}

impl<T> Vec4<T> {
    /// A new vector using the given elements.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Create from `[x, y, z, w]`.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 4]) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { array: ManuallyDrop::new(array) }.vec4 })
    }

    /// Convert to `[x, y, z, w]`.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4] {
        ManuallyDrop::into_inner(unsafe { Cast { vec4: ManuallyDrop::new(self) }.array })
    }

    /// Create from `(x, y, z, w)`.
    #[inline]
    #[must_use]
    pub const fn from_tuple(tuple: (T, T, T, T)) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { tuple: ManuallyDrop::new(tuple) }.vec4 })
    }

    /// Convert to `(x, y, z, w)`.
    #[inline]
    #[must_use]
    pub const fn to_tuple(self) -> (T, T, T, T) {
        ManuallyDrop::into_inner(unsafe { Cast { vec4: ManuallyDrop::new(self) }.tuple })
    }
}

impl<T: Copy> Vec4<T> {
    /// A vector with each component set to `v`.
    #[must_use]
    pub const fn splat(v: T) -> Self {
        Self::from_array([v; 4])
    }

    /// ## Panics
    ///
    /// Panics if the index is not of `0..4`.
    #[inline]
    #[must_use]
    pub const fn get(self, idx: usize) -> T {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!(),
        }
    }
}

// -----------------
// constants and num
// -----------------

impl<T: One> Vec4<T> {
    /// Vector with each component set to `T::ONE`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<T: Zero> Vec4<T> {
    /// Vector with each component set to `T::ZERO`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: NegOne> Vec4<T> {
    /// Vector with each component set to `T::NEG_ONE`.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: One + Zero> Vec4<T> {
    /// A vector with `w` set to one and others set to zero.
    pub const W: Self = Self::with_w(T::ONE);
    /// A vector with `x` set to one and others set to zero.
    pub const X: Self = Self::with_x(T::ONE);
    /// A vector with `y` set to one and others set to zero.
    pub const Y: Self = Self::with_y(T::ONE);
    /// A vector with `z` set to one and others set to zero.
    pub const Z: Self = Self::with_z(T::ONE);

    /// A new vector using `x` and zero for the other components.
    #[inline]
    #[must_use]
    pub const fn with_x(x: T) -> Self {
        let zero = T::ZERO;

        Self::new(x, zero, zero, zero)
    }

    /// A new vector using `y` and zero for the other components.
    #[inline]
    #[must_use]
    pub const fn with_y(y: T) -> Self {
        let zero = T::ZERO;

        Self::new(zero, y, zero, zero)
    }

    /// A new vector using `z` and zero for the other components.
    #[inline]
    #[must_use]
    pub const fn with_z(z: T) -> Self {
        let zero = T::ZERO;

        Self::new(zero, zero, z, zero)
    }

    /// A new vector using `w` and zero for the other components.
    #[inline]
    #[must_use]
    pub const fn with_w(w: T) -> Self {
        let zero = T::ZERO;

        Self::new(zero, zero, zero, w)
    }
}

impl<T: Max> Vec4<T> {
    /// Vector with each component set to `T::MAX`.
    pub const MAX: Self = Self::splat(T::MAX);
}

impl<T: Min> Vec4<T> {
    /// Vector with each component set to `T::MIN`.
    pub const MIN: Self = Self::splat(T::MIN);
}

// ----------
// operations
// ----------

// basic ---

impl<T: Num> Add for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.z)
    }
}

impl<T: Num> Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.z)
    }
}

impl<T: Num> Mul for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.z)
    }
}

impl<T: Num> Div for Vec4<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.z)
    }
}

// assign ---

impl<T: Num> AddAssign for Vec4<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: Num> SubAssign for Vec4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: Num> MulAssign for Vec4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<T: Num> DivAssign for Vec4<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

// -----------
// conversions
// -----------

impl<T> Deref for Vec4<T> {
    type Target = [T; 4];

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl<T> DerefMut for Vec4<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}

impl<T> IntoMint for Vec4<T> {
    type MintType = mint::Vector4<T>;
}

impl<T> From<[T; 4]> for Vec4<T> {
    fn from(array: [T; 4]) -> Self {
        Self::from_array(array)
    }
}

impl<T> From<Vec4<T>> for [T; 4] {
    fn from(vec4: Vec4<T>) -> Self {
        vec4.to_array()
    }
}

impl<T> From<Vec4<T>> for mint::Vector4<T> {
    fn from(vec4: Vec4<T>) -> Self {
        let Vec4 { x, y, z, w } = vec4;

        Self { x, y, z, w }
    }
}

unsafe impl<T: Pod> Pod for Vec4<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec4<T> {}

#[cfg(test)]
mod test {
    use super::super::Vec4;

    #[test]
    fn from_to() {
        let array = [1.0, 2.0, 3.0, 4.0];
        let tuple = (0, 1, 2, 3);

        assert_eq!(Vec4::from_array(array).to_array(), array);
        assert_eq!(Vec4::from_tuple(tuple).to_tuple(), tuple);
    }

    #[test]
    fn fmt() {
        let vec4 = Vec4::splat(0);
        let string = "[ 0 0 0 0 ]";

        assert_eq!(vec4.to_string(), string);
    }
}
