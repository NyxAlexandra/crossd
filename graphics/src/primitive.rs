use bytemuck::{Pod, Zeroable};

use crate::color::Color;
use crate::geometry::{Point2, Size2};

/// A drawable rectangle.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Pod, Zeroable)]
pub struct Quad {
    /// The location of the quad's upper-left corner.
    pub loc: Point2,
    /// The size of the quad.
    pub size: Size2,
    /// The color of the quad's fill.
    pub color: Color,
}

/// A vertex, the building block of larger shapes.
///
/// This type is equivalent in memory to [`Point2`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Pod, Zeroable)]
pub struct Vertex {
    /// The location of the vertex.
    pub loc: Point2,
}

impl Vertex {
    /// A new vertex at the given position.
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { loc: Point2::new(x, y) }
    }

    /// A vertex at the given position.
    #[inline]
    #[must_use]
    pub const fn at(loc: Point2) -> Self {
        Self { loc }
    }
}
