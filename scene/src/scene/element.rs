use std::rc::Rc;

use text::*;

use crate::{Color, Mat3, Path, Point2, Scene, Size2};

mod text;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Element<T> {
    Fill { path: Path<T>, fill: Fill },
    Stroke { path: Path<T>, stroke: Stroke },
    Image { point: Point2<T>, image: Image },
    Text { point: Point2<T>, text: Text },
    Group { trans: Mat3<T>, scene: Scene<T> },
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Fill {
    pub source: Source,
    pub rule: FillRule,
    pub blend: Blend,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Stroke {
    pub source: Source,
    pub rule: FillRule,
    pub blend: Blend,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Source {
    Color(Color),
    Gradient(Gradient),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Gradient {
    Linear { start: Color, end: Color },
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Image {
    pub buffer: ImageBuf,
    pub trans: Mat3<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ImageBuf {
    bytes: Rc<[u8]>,
    size: Size2<u32>,
}

impl Default for Source {
    fn default() -> Self {
        Self::Color(Color::TRANSPARENT)
    }
}

impl ImageBuf {
    pub fn new(bytes: impl AsRef<[u8]>, size: Size2<u32>) -> Option<Self> {
        todo!()
    }

    #[must_use]
    pub fn resize(&mut self, size: Size2<u32>) -> Option<()> {
        todo!()
    }
}
