use crate::graphics::PaintCx;

pub trait Widget {
    fn paint(&mut self, cx: &mut PaintCx);
}
