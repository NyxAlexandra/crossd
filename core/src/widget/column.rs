use crossd_scene::Size2;

use crate::layout::Length;
use crate::view::{LayoutCx, PaintCx, View, ViewCx, ViewSeq, ViewState};
use crate::Event;

pub struct Column<D, S>
where
    S: ViewSeq<D>,
{
    seq: S,
    data: ViewState<D>,

    width: Length,
    height: Length,
}

impl<D, S> Column<D, S>
where
    S: ViewSeq<D>,
{
    pub fn new(seq: S) -> Self {
        Self { seq, data: ViewState::new(()), width: Length::Max, height: Length::Max }
    }

    pub fn width(self, width: Length) -> Self {
        Self { width, ..self }
    }

    pub fn height(self, height: Length) -> Self {
        Self { height, ..self }
    }
}

impl<D, S> View<D> for Column<D, S>
where
    S: ViewSeq<D>,
{
    fn init(&self, cx: &mut ViewCx) -> ViewState<D> {
        ViewState::new(())
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
