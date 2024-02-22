use crossd_scene::Size2;

use crate::view::{LayoutCx, PaintCx, View, ViewCx, ViewState};
use crate::Event;

pub struct Text<D> {
    data: ViewState<D>,
}

impl<D> View<D> for Text<D> {
    fn init(&self, cx: &mut ViewCx) -> ViewState<D> {
        todo!()
    }

    fn reinit(&self, cx: &mut ViewCx, state: &mut ViewState<D>) {
        todo!()
    }

    fn event(
        &mut self,
        cx: &mut ViewCx,
        state: &mut ViewState<D>,
        data: &mut D,
        event: Event,
    ) {
        todo!()
    }

    fn layout(&mut self, cx: &mut LayoutCx) -> Size2<u32> {
        todo!()
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        todo!()
    }
}
