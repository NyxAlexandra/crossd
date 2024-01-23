use crossd_math::Size2;
use wgpu::{
    Extent3d,
    ImageCopyTexture,
    Texture,
    TextureDescriptor,
    TextureDimension,
    TextureUsages,
    TextureView,
};

use self::rasterize::RasterizePipeline;
use crate::backend::{Backend, BackendError};
use crate::encoding::Encoding;
use crate::scene::Scene;

mod rasterize;
pub mod surface;

/// A renderer.
pub struct Graphics {
    backend: Backend,
    output: Texture,
    view: TextureView,

    scene: Scene,
    /// has scene been edited since last render?
    dirty: bool,

    encoding: Encoding,
    pipeline: Pipeline,
}

struct Pipeline {
    rasterize: RasterizePipeline,
}

/// Trait for item that can be rendered to.
pub trait RenderTarget {
    /// Prepare for a new frame.
    ///
    /// This is where [`Graphics::resize`] should be called, if necessary.
    fn prepare(&mut self, graphics: &mut Graphics);

    /// Any special logic can be called here, if necessary.
    fn render(&mut self, graphics: &mut Graphics) {
        graphics.render()
    }

    /// Present to this target to be displayed.
    fn present(&mut self, graphics: &Graphics);
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
        let size = Size2::default();
        let texture = backend.device().create_texture(&TextureDescriptor {
            label: Some("Graphics.output"),
            size: { Extent3d { width: size.w, height: size.h, ..Default::default() } },
            format: backend.format(),
            usage: TextureUsages::STORAGE_BINDING
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC
                | TextureUsages::COPY_DST,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            view_formats: &[],
        });

        todo!()
    }

    /// The [Wgpu backend](Backend).
    pub fn backend(&self) -> &Backend {
        &self.backend
    }

    /// The current size of the viewport.
    pub fn size(&self) -> Size2 {
        let Extent3d { width, height, .. } = self.output.size();

        Size2::new(width, height)
    }

    /// Resize the viewport.
    pub fn resize(&mut self, size: Size2) {
        todo!()
    }

    /// The output texture.
    pub fn output(&self) -> &Texture {
        &self.output
    }

    /// Prepare for rendering a new frame.
    pub fn prepare(&mut self) {
        todo!()
    }

    /// Render to the current texture.
    pub fn render(&mut self) {
        todo!()
    }
}
