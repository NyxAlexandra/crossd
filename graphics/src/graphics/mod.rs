use std::sync::{Arc, RwLock};

use wgpu::{
    CommandEncoderDescriptor,
    Operations,
    RenderPassColorAttachment,
    RenderPassDescriptor,
    TextureAspect,
    TextureViewDescriptor,
    TextureViewDimension,
};

use self::quad::QuadPipeline;
use crate::backend::{Backend, BackendError};
use crate::geometry::Rect;
use crate::primitive::Quad;
use crate::{Frame, Graphics, Target};

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
    pub layers: Vec<Layer>,
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
    /// # fn run() -> anyhow::Result<()> {
    /// let backend =
    ///     Backend::builder().power_preference(PowerPreference::HighPerformance).build()?;
    /// let graphics = Graphics::with(backend);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # run();
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
    pub fn prepare<'frame, T: Target>(
        &mut self,
        target: &'frame T,
    ) -> Result<Frame<'frame, T>, T::Error> {
        let backend = self.backend.clone();
        let context = self.context.clone();

        let view = target.prepare(TextureViewDescriptor {
            label: Some("Frame.view"),
            format: Some(backend.format()),
            ..Default::default()
        })?;

        let mut encoder =
            self.backend.device().create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Frame.encoder"),
            });
        let rpass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Frame.rpass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations { load: todo!(), store: todo!() },
            })],
            depth_stencil_attachment: None,
        });

        Ok(Frame { backend, context, encoder, rpass, target })
    }

    /// Render and present the frame.
    pub fn render<T: Target>(&mut self, frame: Frame<'_, T>) -> Result<(), RenderError> {
        todo!()
    }
}

/// Errors that can arise while [rendering](Graphics::render).
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("placeholder")]
    Placeholder,
}
