use crossd_graphics::math::Size2;

use super::{Element, Painter, Widget};

pub struct Canvas {
    draw: Box<dyn FnMut(&mut Painter)>,
}

impl Canvas {
    pub fn new(draw: impl FnMut(&mut Painter) + 'static) -> Self {
        Self { draw: Box::new(draw) }
    }
}

impl Widget for Canvas {
    fn children(&self) -> Vec<Element> {
        Vec::new()
    }

    fn layout(&mut self, bounds: Size2) -> Size2 {
        todo!()
    }

    fn paint(&mut self, cx: &mut Painter) {
        todo!()
    }
}
