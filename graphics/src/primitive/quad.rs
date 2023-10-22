use super::Quad;
use crate::color::Color;
use crate::geometry::Rect;

impl Quad {
    /// A new quad with the given shape and color.
    #[inline]
    #[must_use]
    pub const fn new(rect: Rect, color: Color) -> Self {
        Self { rect, color }
    }
}
