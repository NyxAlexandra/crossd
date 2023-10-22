#![doc = include_str!("../README.md")]
#![forbid(unsafe_op_in_unsafe_fn)]
// #![warn(missing_docs)]

use std::fmt;
use std::sync::Arc;

use color::Color;
use geometry::Size2;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub extern crate bytemuck;
pub extern crate crossd_math as math;
pub extern crate glyphon;
pub extern crate raw_window_handle;
// `use crossd_graphics` in examples
extern crate self as crossd_graphics;
#[cfg(feature = "trace")]
pub extern crate tracing;
pub extern crate wgpu;

pub mod backend;
pub mod color;
pub mod geometry;
/// Implementations for [`Graphics`], [`Frame`], etc.
mod graphics;
pub mod primitive;
/// Various internal utilities.
mod utils;

/// A renderer.
pub struct Graphics {
    backend: Arc<backend::Backend>,
    context: Arc<graphics::Context>,
}

/// An in-progress frame.
pub struct Frame<'pass, T: Target> {
    backend: Arc<backend::Backend>,
    context: Arc<graphics::Context>,

    rpass: wgpu::RenderPass<'pass>,
    encoder: wgpu::CommandEncoder,

    target: &'pass T,
}

/// A window-backed target.
pub struct Surface<T: SurfaceTarget> {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    target: T,
}

/// Trait for types that can be used with [`Surface`] to be drawn to.
pub trait SurfaceTarget: HasRawWindowHandle + HasRawDisplayHandle {
    /// The size of the surface target.
    fn size(&self) -> Size2<u32>;
}

/// A pixel-buffer-backed [target](Targetable).
pub struct Canvas {
    /// Dimensions of the buffer.
    size: Size2,
    /// The pixels in the buffer.
    pixels: Vec<Color>,
}

/// Trait for item that can be rendered to.
pub trait Target {
    /// Error that can arise when presenting.
    type Error: fmt::Debug;

    /// Size of the drawable area in pixels.
    fn size(&self) -> Size2<u32>;

    /// Present to this target to be displayed.
    fn present(
        &self, // something here, I don't know what yet
    ) -> Result<(), Self::Error>;
}

/// Trait for items that can be drawn.
pub trait Draw {
    /// Draw this item.
    fn draw(frame: &mut Frame<'_, impl Target>, item: Self);
}
