use crossd_math::{Point2, Rect, Size2};
use crossd_scene::{Fill, Image, Path, Stroke};

use crate::graphics::Renderer;
use crate::layout::Dim2;

pub mod canvas;
pub mod label;

pub trait Widget {
    /// (Possibly)-relative dimensions of this widget.
    fn dimensions(&self) -> Dim2;

    /// Draw this widget.
    fn paint(&self, cx: &mut PaintCx, bounds: Rect<u32>);
}

pub struct Cx {
    renderer: Box<dyn Renderer>,
}

pub struct PaintCx<'a> {
    renderer: &'a mut dyn Renderer,
    bounds: Rect<u32>,
    window: Size2,
}

impl<'a> PaintCx<'a> {
    pub(crate) fn new(renderer: &'a mut impl Renderer, window: Size2) -> Self {
        Self { renderer, bounds: window.to_rect(), window }
    }

    pub fn paint(&mut self, widget: &impl Widget) {
        let bounds = self.bounds;

        self.bounds = widget.dimensions().resolve(bounds.size).to_rect();

        widget.paint(self, bounds);

        self.bounds = bounds;
    }

    pub fn stroke(&mut self, path: impl Into<Path>, stroke: Stroke) {
        self.renderer.stroke(path.into(), stroke)
    }

    pub fn fill(&mut self, path: impl Into<Path>, fill: Fill) {
        self.renderer.fill(path.into(), fill)
    }

    pub fn image(&mut self, point: Point2, image: Image) {
        self.renderer.image(point, image)
    }
}

impl<'a> Renderer for PaintCx<'a> {
    fn stroke(&mut self, path: Path, stroke: Stroke) {
        self.stroke(path, stroke)
    }

    fn fill(&mut self, path: Path, fill: Fill) {
        self.fill(path, fill)
    }

    fn image(&mut self, point: Point2, image: Image) {
        self.image(point, image)
    }
}
