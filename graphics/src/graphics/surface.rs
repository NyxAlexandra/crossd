use std::iter;

use crossd_math::Size2;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{CommandEncoderDescriptor, ImageCopyTexture};
#[cfg(feature = "winit")]
use winit::window::Window;

use crate::graphics::{Graphics, RenderTarget};

/// A window-backed target.
pub struct Surface<T: SurfaceTarget> {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    texture: Option<wgpu::SurfaceTexture>,

    target: T,
}

/// Trait for types that can be used by a [`Surface`] for rendering.
pub trait SurfaceTarget: HasRawWindowHandle + HasRawDisplayHandle {
    /// The current size of this surface, in pixels.
    fn size(&self) -> Size2;
}

impl<T: SurfaceTarget> Surface<T> {
    /// Create a new surface from a [`SurfaceTarget`].
    pub fn new(target: T, graphics: &mut Graphics) -> Result<Self, NewError> {
        let size = target.size();

        graphics.resize(size);

        let surface = unsafe { graphics.backend.instance().create_surface(&target) }?;
        let config = surface
            .get_default_config(graphics.backend.adapter(), size.w, size.h)
            .ok_or(NewError::UnsupportedSurface)?;
        let texture = None;

        Ok(Self { surface, config, texture, target })
    }

    /// The target this surface wraps.
    pub fn target(&self) -> &T {
        &self.target
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewError {
    #[error(transparent)]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error("surface unsupported")]
    UnsupportedSurface,
}

impl<T: SurfaceTarget> RenderTarget for Surface<T> {
    fn prepare(&mut self, graphics: &mut Graphics) {
        let new_size = self.target.size();
        let old_size = Size2::new(self.config.width, self.config.height);

        if new_size != old_size {
            self.config.width = new_size.w;
            self.config.height = new_size.h;

            self.surface.configure(graphics.backend().device(), &self.config);
            graphics.resize(new_size);
        }

        self.texture = self.surface.get_current_texture().ok();
    }

    fn present(&mut self, graphics: &Graphics) {
        if let Some(texture) = self.texture.take() {
            let backend = graphics.backend();

            let mut encoder =
                backend.device().create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Surface::present#encoder"),
                });

            encoder.copy_texture_to_texture(
                graphics.output().as_image_copy(),
                texture.texture.as_image_copy(),
                texture.texture.size(),
            );
            backend.queue().submit(iter::once(encoder.finish()));
            texture.present();
        }
    }
}

#[cfg(feature = "winit")]
impl SurfaceTarget for Window {
    fn size(&self) -> Size2 {
        use winit::dpi::PhysicalSize;

        let PhysicalSize { width, height } = self.inner_size();

        Size2::new(width, height)
    }
}
