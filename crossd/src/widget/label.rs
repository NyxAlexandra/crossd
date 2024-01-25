use crossd_math::Size2;
use crossd_scene::Text;

use crate::layout::Length;

/// A widget displaying a piece of [`Text`].
pub struct Label {
    text: Text,
    size: Size2<Length>,
}

impl Label {
    /// Create a new label with one piece of text.
    pub fn new(text: Text) -> Self {
        Self { text, size: Size2::new(Length::Fill, Length::Fill) }
    }

    pub fn with_width(self, w: Length) -> Self {
        Self { size: Size2 { w, ..self.size }, ..self }
    }

    pub fn with_height(self, h: Length) -> Self {
        Self { size: Size2 { h, ..self.size }, ..self }
    }
}
