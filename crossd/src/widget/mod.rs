use crate::view::View;

pub mod column;
pub mod row;
pub mod text;

pub trait Widget<D>: View<D> {
    fn layout(&mut self, cx: &mut LayoutCx);

    fn paint(&mut self, cx: &mut PaintCx);
}

pub struct LayoutCx {}

pub struct PaintCx {}
