use super::{Layer, Scene};
use crate::primitive::Quad;

impl Scene {
    /// A new empty scene.
    pub fn new() -> Self {
        Self::default()
    }

    /// The current layer.
    pub fn current(&self) -> &Layer {
        &self.scenes[self.current]
    }

    /// The current layer.
    pub fn current_mut(&mut self) -> &mut Layer {
        &mut self.scenes[self.current]
    }
}
