use crossd_math::{Point2, Size2};
use crossd_scene::{Fill, Image, Path, Stroke};

use crate::graphics::Renderer;

pub mod canvas;

pub trait Widget {
    fn children(&self) -> Vec<Element>;

    fn layout(&mut self, bounds: Size2) -> Size2;

    fn paint(&mut self, cx: &mut Painter);
}

/// A generic widget.
pub struct Element {
    widget: Box<dyn Widget>,
}

pub struct Cx {
    window_size: Size2,
    fullscreen: bool,
}

pub struct Painter {
    renderer: Box<dyn Renderer>,
}

impl Element {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self { widget: Box::new(widget) }
    }

    pub fn widget(&self) -> &dyn Widget {
        self.widget.as_ref()
    }

    pub fn widget_mut(&mut self) -> &mut dyn Widget {
        self.widget.as_mut()
    }
}

impl Widget for Element {
    fn children(&self) -> Vec<Element> {
        self.widget.children()
    }

    fn layout(&mut self, bounds: Size2) -> Size2 {
        self.widget.layout(bounds)
    }

    fn paint(&mut self, cx: &mut Painter) {
        self.widget.paint(cx)
    }
}

impl Painter {
    pub(crate) fn new(renderer: impl Renderer + 'static) -> Self {
        Self { renderer: Box::new(renderer) }
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

impl Renderer for Painter {
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
