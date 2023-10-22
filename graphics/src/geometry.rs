//! Simple geometry types usable on the GPU.

/// Implementations for [`Point2`].
mod point2;
/// Implementations for [`Rect`].
mod rect;
/// Implementations for [`Size2`].
mod size2;

/// A point in 2 dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point2<T = f32> {
    pub x: T,
    pub y: T,
}

/// A rectangle defined by it's location and size.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect<T = f32> {
    pub loc: Point2<T>,
    pub size: Size2<T>,
}

/// A size in 2 dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size2<T = u32> {
    pub w: T,
    pub h: T,
}
