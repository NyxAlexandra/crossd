use std::sync::{Arc, RwLock};

use wgpu::{CommandEncoderDescriptor, RenderPass, SurfaceError};

use self::quad::QuadPipeline;
use crate::backend::{Backend, NewError, RenderError};
use crate::geometry::Rect;
use crate::primitive::Quad;
use crate::{Draw, Frame, Graphics, Target};

mod canvas;
mod quad;
mod surface;
mod triangle;

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
    pub fn new() -> Result<Self, NewError> {
        pollster::block_on(Backend::new()).map(Self::with)
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
    pub fn render<T: Target>(&mut self, frame: Frame<'_, T>) -> Result<(), RenderError> {
        todo!()
    }
}

impl Context {
    /// Initializes drawing state.
    pub fn new(backend: &Backend) -> Self {
        let quad = QuadPipeline::new(&backend);
        let scene = Scene::default();

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

impl<'frame, T: Target> Frame<'frame, T> {
    fn new(graphics: &Graphics, rpass: RenderPass<'frame>, target: &'frame T) -> Self {
        let backend = graphics.backend.clone();
        let context = graphics.context.clone();
        let encoder = graphics
            .backend
            .device()
            .create_command_encoder(&CommandEncoderDescriptor::default());

        Self { backend, context, rpass, encoder, target }
    }

    /// Draw an item.
    pub fn draw(&mut self, item: impl Draw) {
        Draw::draw(self, item)
    }
}

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

impl Layer {
    /// Create a new empty layer with the given clip bounds.
    pub fn new(bounds: Rect<u32>) -> Self {
        Self { bounds, quads: Vec::new() }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PresentError {
    #[error(transparent)]
    SurfaceError(#[from] SurfaceError),
}
