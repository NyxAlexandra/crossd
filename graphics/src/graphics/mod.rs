use std::sync::{Arc, RwLock};

use self::quad::QuadPipeline;
use crate::backend::{Backend, BackendError};
use crate::geometry::Rect;
use crate::primitive::Quad;
use crate::{Frame, Graphics, Target};

mod canvas;
mod context;
mod frame;
mod layer;
/// Quad render pipeline.
mod quad;
mod scene;
mod surface;

/// Shared drawing state.
pub struct Context {
    inner: RwLock<Inner>,
}

/// Inner [`Context`].
pub struct Inner {
    /// Pipeline for rendering colored "quads" (rectangles).
    pub quad: QuadPipeline,
    /// "Scene Graph".
    pub scene: Scene,
}

/// A collection of [`Layer`]s.
#[derive(Debug, Default, Clone)]
pub struct Scene {
    /// Groups of primitives.
    pub scenes: Vec<Layer>,
    /// The current scene.
    pub current: usize,
}

/// A group of primitives.
#[derive(Debug, Default, Clone)]
pub struct Layer {
    /// Clip bounds.
    pub bounds: Rect<u32>,
    /// Quads in this layer.
    pub quads: Vec<Quad>,
}

impl Graphics {
    /// Create a new renderer.
    ///
    /// ```
    /// # use crossd_graphics::Graphics;
    /// #
    /// # fn run() -> anyhow::Result<()> {
    /// let graphics = Graphics::new().await?;
    /// #
    /// #     Ok(())
    /// # }
    /// # run();
    /// ```
    pub fn new() -> Result<Self, BackendError> {
        Backend::new().map(Self::with)
    }

    /// A graphics backend using the provided backend.
    ///
    /// ```
    /// # use crossd_graphics::{Graphics, Backend};
    /// # use crossd_graphics::wgpu::PowerPreference;
    /// #
    /// # async fn run() -> anyhow::Result<()> {
    /// let backend = Backend::builder()
    ///     .power_preference(PowerPreference::HighPerformance)
    ///     .build()
    ///     .await?;
    /// let graphics = Graphics::with(backend);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # pollster::block_on(run());
    /// ```
    pub fn with(backend: Backend) -> Self {
        let backend = Arc::new(backend);
        let context = Arc::new(Context::new(&backend));

        Self { backend, context }
    }

    /// Any changes to the internal state of the backend may cause issues.
    /// Stability across different version of the library is not guaranteed.
    pub fn backend(&self) -> &Backend {
        &self.backend
    }

    /// Prepare the rendering of a new frame.
    pub fn prepare<'frame, T: Target>(&mut self, target: &'frame T) -> Frame<'frame, T> {
        todo!()
    }

    /// Render and present the frame.
    pub fn render<T: Target>(
        &mut self,
        frame: Frame<'_, T>,
    ) -> Result<(), RenderError<T>> {
        todo!()
    }
}

/// Errors that can arise while [rendering](Graphics::render).
#[derive(Debug, thiserror::Error)]
pub enum RenderError<T: Target> {
    #[error("target error: {0:?}")]
    TargetError(T::Error),
}
