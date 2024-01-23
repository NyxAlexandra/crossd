use crossd_math::{Size2, Trans2};

use super::color::Color;
use super::path::Path;

/// A basic building-block of a vector scene.
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Fill { path: Path, fill: Fill },
    Stroke { path: Path, stroke: Stroke },
    Group { trans: Trans2, members: Vec<Element> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fill {
    pub source: Source,
    pub rule: FillRule,
    pub blend: Blend,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub width: f32,
    pub source: Source,
    pub blend: Blend,
    pub join: Join,
    pub cap: Cap,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub bytes: Vec<u8>,
    pub size: Size2,
    pub trans: Trans2,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// A solid [color](Color).
    Solid(Color),
    Gradient(Gradient),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FillRule {
    EvenOdd,
    NonZero,
}

/// Methods for blending colors.
///
/// ![](https://fiddle.skia.org/i/819903e0bb125385269948474b6c8a84_raster.png)
///
/// See [skia.org - SkBlendMode Overview](https://skia.org/docs/user/api/skblendmode_overview) for a full explanation.
/// (Not all modes are implemented.)
#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Blend {
    /// Just source.
    Src,
    /// Just destination.
    Dst,
    /// (default) Source over destination.
    SrcOver,
    /// Destination over source.
    DstOver,
}

/// Styles of joining lines.
#[derive(Debug, Clone, PartialEq)]
pub enum Join {
    Round,
    Bevel,
    Miter { limit: f32 },
}

/// Styles of ending lines.
#[derive(Debug, Clone, PartialEq)]
pub enum Cap {
    Butt,
    Round,
    Square,
}

impl Fill {
    pub const DEFAULT: Self =
        Self { source: Source::DEFAULT, rule: FillRule::DEFAULT, blend: Blend::DEFAULT };

    pub const fn from_color(color: Color) -> Self {
        Self { source: Source::Solid(color), ..Self::DEFAULT }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Stroke {
    pub const DEFAULT: Self = Self {
        width: 0.0,
        join: Join::DEFAULT,
        cap: Cap::DEFAULT,
        source: Source::DEFAULT,
        blend: Blend::DEFAULT,
    };

    pub const fn from_color(color: Color) -> Self {
        Self { source: Source::Solid(color), ..Self::DEFAULT }
    }

    pub const fn with_width(self, width: f32) -> Self {
        Self { width, ..self }
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Image {
    /// Bytes for a single pixel.
    const BYTES_PER_PIXEL: usize = 4;
    /// The default image, no bytes and of size 0 with an identity transform.
    pub const DEFAULT: Self = Self {
        bytes: Vec::new(),
        size: Size2::ZERO,
        trans: Trans2::IDENTITY,
        opacity: 1.0,
    };

    /// A new image with the given bytes.
    pub fn new(bytes: &[u8], size: Size2) -> Option<Self> {
        ((size.w * size.h) as usize * Self::BYTES_PER_PIXEL < bytes.len()).then(|| Self {
            bytes: bytes.to_vec(),
            size,
            ..Self::DEFAULT
        })
    }
}

impl Default for Image {
    fn default() -> Self {
        Image::DEFAULT
    }
}

impl Source {
    pub const DEFAULT: Self = Self::Solid(Color::BLACK);
}

impl Default for Source {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl FillRule {
    pub const DEFAULT: Self = Self::EvenOdd;
}

impl Default for FillRule {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Blend {
    pub const DEFAULT: Self = Self::SrcOver;
}

impl Default for Blend {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Join {
    pub const DEFAULT: Self = Self::Miter { limit: 10.0 };
}

impl Default for Join {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Cap {
    pub const DEFAULT: Self = Self::Butt;
}

impl Default for Cap {
    fn default() -> Self {
        Self::DEFAULT
    }
}
