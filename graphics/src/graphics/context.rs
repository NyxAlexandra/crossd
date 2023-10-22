use std::sync::RwLock;

use super::quad::QuadPipeline;
use super::{Context, Inner, Scene};
use crate::backend::Backend;

impl Context {
    /// Initializes drawing state.
    pub fn new(backend: &Backend) -> Self {
        let quad = QuadPipeline::new(&backend);
        let scene = Scene::new();

        Self { inner: RwLock::new(Inner { quad, scene }) }
    }

    /// With [`&Inner`](Inner).
    pub fn with<R>(&self, f: impl FnOnce(&Inner) -> R) -> R {
        f(&self.inner.read().unwrap())
    }

    /// With [`&mut Inner`](Inner).
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut Inner) -> R) -> R {
        f(&mut self.inner.write().unwrap())
    }
}
