use wgpu::SurfaceConfiguration;
#[cfg(feature = "winit")]
use winit::{dpi::PhysicalSize, window::Window};

use crate::geometry::Size2;
use crate::{Graphics, Surface, SurfaceTarget, Target};

#[derive(Debug, thiserror::Error)]
pub enum NewError {
    #[error(transparent)]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error("surface unsupported")]
    UnsupportedSurface,
}

impl<T: SurfaceTarget> Surface<T> {
    /// Create a new surface from a [`SurfaceTarget`].
    pub fn new(graphics: &Graphics, target: T) -> Result<Self, NewError> {
        use NewError::*;

        let surface = unsafe { graphics.backend.instance().create_surface(&target) }?;
        let size = target.size();
        let config = SurfaceConfiguration {
            width: size.w,
            height: size.h,
            ..surface
                .get_default_config(graphics.backend.adapter(), 1, 1)
                .ok_or(UnsupportedSurface)?
        };

        Ok(Self { surface, config, target })
    }

    /// The size of the surface target.
    pub fn size(&self) -> Size2 {
        self.target.size()
    }
}

impl<T: SurfaceTarget> Target for Surface<T> {
    fn size(&self) -> Size2<u32> {
        self.target.size()
    }

    fn present(&self) {
        todo!()
    }
}

#[cfg(feature = "winit")]
impl SurfaceTarget for Window {
    fn size(&self) -> Size2<u32> {
        let PhysicalSize { width, height } = self.inner_size();

        Size2::new(width, height)
    }
}
