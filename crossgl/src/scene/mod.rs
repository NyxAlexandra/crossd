use std::mem;

use bytemuck::{Pod, Zeroable};
pub use element::*;
pub use gpu::*;
pub use path::*;

use crate::math::{Mat3, Point2, Rect};

mod element;
mod gpu;
mod path;

/// Encodes drawing operations for the GPU.
#[derive(Debug)]
pub struct Scene {
    /// Layout of a path:
    ///
    /// ```text
    /// (len (MOVE|LINE|QUAD|CUBIC points...))...
    /// ```
    paths: BufferVec<u32>,
    /// Flattened path segments.
    ///
    /// ```text
    /// (start end)...
    /// ```
    lines: BufferVec<Point2<f32>>,
    /// Encoded drawing operations.
    draw_ops: BufferVec<DrawOp>,
    styles: BufferVec<Style>,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct DrawOp(u32);

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Style {
    /// Packed fill and stroke information.
    flags: u32,
    /// - [`Source::Color`]: `[color, 0]`
    /// - [`Source::Gradient`]:
    ///   - [`Gradient::Linear`]: `[start, end]`
    ///   - [`Gradient::Radial`]: `[inner, outer]`
    source: [u32; 2],
    /// Stroke width.
    ///
    /// Unused if fill.
    width: f32,
}

impl Scene {
    pub fn new(cx: &Context) -> Self {
        let paths = BufferVec::new(cx, BufferBuilder::default(), 0);
        let lines = BufferVec::new(cx, BufferBuilder::default(), 0);
        let draw_ops = BufferVec::new(cx, BufferBuilder::default(), 0);
        let styles = BufferVec::new(cx, BufferBuilder::default(), 0);

        Self { paths, lines, draw_ops, styles }
    }

    pub fn fill(&mut self, path: impl Into<Path>, fill: &Fill) {
        todo!()
    }

    pub fn stroke(&mut self, path: impl Into<Path>, stroke: &Stroke) {
        todo!()
    }

    pub fn image(&mut self, point: impl Into<Point2<f32>>, image: &Image) {
        todo!()
    }

    pub fn texture(&mut self, point: impl Into<Point2<f32>>, texture: &Texture) {
        todo!()
    }

    /// Clip subsequent drawing commands.
    pub fn push_clip(&mut self, clip: Rect<f32>) {
        todo!()
    }

    /// Pop the previous clip.
    pub fn pop_clip(&mut self) -> Option<Rect<f32>> {
        todo!()
    }

    /// Transform subsequent drawing commands.
    pub fn push_transform(&mut self, trans: Mat3<f32>) {
        todo!()
    }

    /// Pop the previous transform.
    pub fn pop_transform(&mut self) -> Option<Mat3<f32>> {
        todo!()
    }

    /// Remove all elements from the scene.
    pub fn clear(&mut self) {
        self.paths.vec_mut().clear();
        self.lines.vec_mut().clear();
        self.styles.vec_mut().clear();
    }

    fn encode_path(&mut self, path: &Path) -> u32 {
        const MOVE: u32 = 0;
        const CUBIC: u32 = 1;
        const LINE: u32 = 2;
        const QUAD: u32 = 3;
        const CLOSE: u32 = 4;

        let paths = self.paths.vec_mut();
        let idx = paths.len() as _;

        paths.push((path.verbs().len() + path.points().len()) as _);

        for elem in path {
            match elem {
                PathElement::MoveTo(Point2 { x, y }) => {
                    paths.extend([MOVE, x.to_bits(), y.to_bits()]);
                },
                PathElement::LineTo(Point2 { x, y }) => {
                    paths.extend([LINE, x.to_bits(), y.to_bits()]);
                },
                PathElement::QuadTo { p, c } => {
                    paths.push(QUAD);
                    paths.extend(unsafe { mem::transmute::<_, [u32; 2]>(p) });
                    paths.extend(unsafe { mem::transmute::<_, [u32; 2]>(c) });
                },
                PathElement::CubicTo { p, c1, c2 } => {
                    paths.push(CUBIC);
                    paths.extend(unsafe { mem::transmute::<_, [u32; 2]>(p) });
                    paths.extend(unsafe { mem::transmute::<_, [u32; 2]>(c1) });
                    paths.extend(unsafe { mem::transmute::<_, [u32; 2]>(c2) });
                },
                PathElement::Close => paths.push(CLOSE),
            }
        }

        idx
    }

    fn encode_style(&mut self, style: impl Into<Style>) -> u32 {
        let idx = self.styles.vec().len() as _;

        self.styles.vec_mut().push(style.into());

        idx
    }
}

#[rustfmt::skip]
impl Style {
    const FILL: u32 = 0b0;
    const STROKE: u32 = 0b0;

    const SOURCE_COLOR: u32 = 0b0_0;
    const SOURCE_GRADIENT_LINEAR: u32 = 0b1_0;
    const SOURCE_GRADIENT_RADIAL: u32 = 0b10_0;

    const FILL_RULE_EVEN_ODD: u32 = 0b0_000;
    const FILL_RULE_NON_ZERO: u32 = 0b1_000;

    const BLEND_SRC_OVER: u32 = 0b0_0000;
    const BLEND_DST_OVER: u32 = 0b1_0000;
    const BLEND_SRC: u32 = 0b10_0000;
    const BLEND_DST: u32 = 0b11_0000;
}

impl From<Fill> for Style {
    fn from(Fill { source, rule, blend }: Fill) -> Self {
        let mut flags = Self::FILL;

        let source = match source {
            Source::Color(color) => {
                flags |= Self::SOURCE_COLOR;

                [color.into(), 0]
            },
            Source::Gradient(Gradient::Linear { start, end }) => {
                flags |= Self::SOURCE_GRADIENT_LINEAR;

                [start.into(), end.into()]
            },
            Source::Gradient(Gradient::Radial { inner, outer }) => {
                flags |= Self::SOURCE_GRADIENT_RADIAL;

                [inner.into(), outer.into()]
            },
        };

        match rule {
            FillRule::EvenOdd => flags |= Self::FILL_RULE_EVEN_ODD,
            FillRule::NonZero => flags |= Self::FILL_RULE_NON_ZERO,
        };

        match blend {
            Blend::Src => flags |= Self::BLEND_SRC,
            Blend::Dst => flags |= Self::BLEND_DST,
            Blend::SrcOver => flags |= Self::BLEND_SRC_OVER,
            Blend::DstOver => flags |= Self::BLEND_DST_OVER,
        };

        Self { flags, source, width: 0.0 }
    }
}

impl From<Stroke> for Style {
    fn from(Stroke { fill, width }: Stroke) -> Self {
        let mut style = Self { width, ..Self::from(fill) };

        style.flags |= Self::STROKE;

        style
    }
}
