use super::{Layer, Scene};
use crate::primitive::Quad;

impl Scene {
    /// A new empty scene.
    pub fn new() -> Self {
        Self::default()
    }

    /// The current layer.
    pub fn current(&self) -> &Layer {
        &self.layers[self.current]
    }

    /// The current layer.
    pub fn current_mut(&mut self) -> &mut Layer {
        &mut self.layers[self.current]
    }
}
