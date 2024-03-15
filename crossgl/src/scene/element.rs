use std::borrow::Cow;

use wgpu::TextureFormat;

use crate::color::Color;
use crate::math::Size2;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Fill {
    pub source: Source,
    pub rule: FillRule,
    pub blend: Blend,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Stroke {
    pub fill: Fill,
    pub width: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Source {
    Color(Color),
    Gradient(Gradient),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Gradient {
    Linear { start: Color, end: Color },
    Radial { inner: Color, outer: Color },
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub enum FillRule {
    #[default]
    EvenOdd,
    NonZero,
}

/// Methods for blending colors.
///
/// ![](https://fiddle.skia.org/i/819903e0bb125385269948474b6c8a84_raster.png)
///
/// See [skia.org - SkBlendMode Overview](https://skia.org/docs/user/api/skblendmode_overview)
/// for a full explanation. (Not all modes are implemented.)
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Blend {
    /// Just source.
    Src,
    /// Just destination.
    Dst,
    /// (default) Source over destination.
    #[default]
    SrcOver,
    /// Destination over source.
    DstOver,
}

/// Texture data on the CPU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub data: Cow<'static, [u8]>,
    pub size: Size2<u32>,
    pub format: TextureFormat,
}

impl Default for Source {
    fn default() -> Self {
        Self::Color(Color::TRANSPARENT)
    }
}
