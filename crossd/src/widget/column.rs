use crate::layout::Length;
use crate::view::{View, ViewCx, ViewSeq, ViewState};
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
        todo!()
    }

    fn reinit(&self, cx: &mut ViewCx, state: &mut ViewState<D>) {
        todo!()
    }

    fn event(
        &mut self,
        cx: &mut ViewCx,
        state: &mut ViewState<D>,
        event: Event,
        data: &mut D,
    ) -> Option<()> {
        todo!()
    }
}
