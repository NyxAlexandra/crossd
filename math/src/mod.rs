//! Various math types.

pub use num::*;

/// Implementations for [`Mat4`].
mod mat4;
/// Const number traits.
mod num;
/// Implementations for [`Vec2`].
mod vec2;
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
/// This type is equivalent to `[T; 4]`, which it also dereferences to for
/// indexing with `vec4[idx]`.
///
/// Conversion to `[T; 4]` occurs in-place.
///
/// Additionally, in WGSL this is `vec4<T>`.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub struct Vec2<T = f32> {
    /// The `x` component.
    pub x: T,
    /// The `y` component.
    pub y: T,
}

/// Shorthand for [`Mat4::new`].
#[inline]
#[must_use]
pub const fn mat4<T>(x: [T; 4], y: [T; 4], z: [T; 4], w: [T; 4]) -> Mat4<T> {
    Mat4::new(x, y, z, w)
}

/// Shorthand for [`Vec4::new`].
#[inline]
#[must_use]
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new(x, y, z, w)
}
