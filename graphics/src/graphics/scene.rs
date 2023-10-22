use super::Scene;
use crate::primitive::Quad;

impl Scene {
    /// A new empty scene.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a quad to the current layer.
    pub fn add_quad(&mut self, quad: Quad) {
        self.scenes[self.current].quads.push(quad)
    }
}
