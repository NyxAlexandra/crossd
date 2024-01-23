#![doc = include_str!("../README.md")]
#![forbid(unsafe_op_in_unsafe_fn)]
// #![warn(missing_docs)]

pub extern crate crossd_math as math;
pub extern crate glyphon;
pub extern crate raw_window_handle;
// `use crossd_graphics` in examples
extern crate self as crossd_graphics;
#[cfg(feature = "trace")]
pub extern crate tracing;
pub extern crate wgpu;

pub use crossd_scene as scene;

pub mod backend;
mod encoding;
pub mod graphics;
/// Various internal utilities.
mod utils;
