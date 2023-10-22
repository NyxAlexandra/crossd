use super::Layer;
use crate::geometry::Rect;
use crate::primitive::Quad;

impl Layer {
    /// Create a new empty layer with the given clip bounds.
    pub fn new(bounds: Rect<u32>) -> Self {
        Self { bounds, quads: Vec::new() }
    }

    /// Add a quad to this layer.
    pub fn add_quad(&mut self, quad: Quad) {
        self.quads.push(quad)
    }
}
