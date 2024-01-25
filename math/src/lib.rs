//! Various math types.

pub use num::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Implementations for [`Mat2`].
mod mat2;
/// Implementations for [`Mat3x2`].
mod mat3x2;
/// Implementations for [`Mat4`].
mod mat4;
/// Const number traits.
mod num;
/// Implementations for [`Point2`].
mod point2;
/// Implementations for [`Rect`].
mod rect;
/// Implementations for [`Size2`].
mod size2;
/// Implementations for [`Trans2`].
mod trans2;
/// Implementations for [`Vec2`].
mod vec2;
/// Implementations for [`Vec3`].
mod vec3;
/// Implementations for [`Vec4`].
mod vec4;

/// A 4x4 column-major matrix.
///
/// ## Repr
///
/// This type is equivalent to:
///
/// - `[Vec4<T>; 4]`
/// - `[[T; 4]; 4]`
/// - `[T; 16]`
/// - [`mint::ColumnMatrix4<T>`]
/// - WGSL: `mat4x4<T>`
///
/// Conversion of `Mat4<T>` to any of the above types occurs in-place.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat4<T = f32> {
    /// The `x` column.
    pub x: Vec4<T>,
    /// The `y` column.
    pub y: Vec4<T>,
    /// The `z` column.
    pub z: Vec4<T>,
    /// The `w` column.
    pub w: Vec4<T>,
}

/// A 4-dimensional vector.
///
/// ## Ops
///
/// This type implements `Add`, `Sub`, `Mul`, and `Div` (+ their -`Assign`
/// counterparts) elementwise. This requires `T: Num`.
///
/// ## Repr
///
/// This type is equivalent to `[T; 4]`.
///
/// Conversion to `[T; 4]` occurs in-place.
///
/// Additionally, in WGSL this is `vec4<T>`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec4<T = f32> {
    /// The `x` component.
    pub x: T,
    /// The `y` component.
    pub y: T,
    /// The `z` component.
    pub z: T,
    /// The `w` component.
    pub w: T,
}

/// A 4x4 column-major matrix.
///
/// ## Repr
///
/// This type is equivalent to:
///
/// - `[Vec3<T>; 3]`
/// - `[[T; 3]; 3]`
/// - `[T; 9]`
/// - [`mint::ColumnMatrix3<T>`]
/// - WGSL: `mat3x3<T>`
///
/// Conversion of `Mat3<T>` to any of the above types occurs in-place.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat3<T = f32> {
    /// The `x` column.
    pub x: Vec3<T>,
    /// The `y` column.
    pub y: Vec3<T>,
    /// The `z` column.
    pub z: Vec3<T>,
}

/// A 3-dimensional vector.
///
/// ## Ops
///
/// This type implements `Add`, `Sub`, `Mul`, and `Div` (+ their -`Assign`
/// counterparts) elementwise. This requires `T: Num`.
///
/// ## Repr
///
/// This type is equivalent to `[T; 3]`, which it also dereferences to for
/// indexing with `vec3[idx]`.
///
/// Conversion to `[T; 3]` occurs in-place.
///
/// Additionally, in WGSL this is `vec3<T>`.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec3<T = f32> {
    /// The `x` component.
    pub x: T,
    /// The `y` component.
    pub y: T,
    /// The `z` component.
    pub z: T,
}

/// A 3x2 column-major matrix.
///
/// ## Ops
///
/// This type implements `Add`, `Sub`, `Mul`, and `Div` (+ their -`Assign`
/// counterparts) elementwise. This requires `T: Num`.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat3x2<T = f32> {
    /// The x column.
    pub x: Vec2<T>,
    /// The y column.
    pub y: Vec2<T>,
    /// The z column.
    pub z: Vec2<T>,
}

/// A 3x2 column-major affine transform matrix.
///
/// ## Repr
///
/// This type is identical to [`Mat3x2<T>`](Mat3x2) with different fields.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trans2<T = f32> {
    /// The matrix component.
    pub mat2: Mat2<T>,
    /// The translation component.
    pub trans: Vec2<T>,
}

/// A 2x2 column-major matrix.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat2<T = f32> {
    /// The `x` column.
    pub x: Vec2<T>,
    /// The `y` column.
    pub y: Vec2<T>,
}

/// A 2-dimensional vector.
///
/// ## Ops
///
/// This type implements `Add`, `Sub`, `Mul`, and `Div` (+ their -`Assign`
/// counterparts) elementwise. This requires `T: Num`.
///
/// ## Repr
///
/// This type is equivalent to `[T; 2]`, which it also dereferences to for
/// indexing with `vec2[idx]`.
///
/// Conversion to `[T; 2]` occurs in-place.
///
/// Additionally, in WGSL this is `vec2<T>`.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vec2<T = f32> {
    /// The `x` component.
    pub x: T,
    /// The `y` component.
    pub y: T,
}

/// A point in 2 dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point2<T = f32> {
    pub x: T,
    pub y: T,
}

/// A rectangle defined by it's location and size.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T = f32> {
    pub loc: Point2<T>,
    pub size: Size2<T>,
}

/// A size in 2 dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Size2<T = u32> {
    pub w: T,
    pub h: T,
}

/// Shorthand for [`Mat4::new`].
#[inline]
#[must_use]
pub const fn mat4<T: Num>(x: [T; 4], y: [T; 4], z: [T; 4], w: [T; 4]) -> Mat4<T> {
    Mat4::new(x, y, z, w)
}

/// Shorthand for [`Vec4::new`].
#[inline]
#[must_use]
pub const fn vec4<T: Num>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new(x, y, z, w)
}

/// Shorthand for [`Rect::new`].
#[inline]
#[must_use]
pub const fn rect<T: Num>(loc: Point2<T>, size: Size2<T>) -> Rect<T> {
    Rect::new(loc, size)
}

/// Shorthand for [`Point2::new`].
#[inline]
#[must_use]
pub const fn point2<T: Num>(x: T, y: T) -> Point2<T> {
    Point2::new(x, y)
}

/// Shorthand for [`Size2::new`].
#[inline]
#[must_use]
pub const fn size2<T: Num>(w: T, h: T) -> Size2<T> {
    Size2::new(w, h)
}

/// Shorthand for [`Vec2::new`].
#[inline]
#[must_use]
pub const fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}
