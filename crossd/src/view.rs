use std::any::Any;
use std::marker::PhantomData;

use crossd_math::Size2;

use crate::Event;

pub trait View<D> {
    fn init(&self, cx: &mut ViewCx) -> ViewState<D>;

    fn reinit(&self, cx: &mut ViewCx, state: &mut ViewState<D>);

    fn event(
        &mut self,
        cx: &mut ViewCx,
        state: &mut ViewState<D>,
        event: Event,
        data: &mut D,
    );

    fn layout(&mut self, cx: &mut ViewCx) -> Size2;

    fn update(&mut self, cx: &mut ViewCx, state: &mut ViewState<D>);
}

pub struct ViewCx {}

pub struct ViewState<D> {
    state: Box<dyn Any>,
    _data: PhantomData<D>,
}

pub trait ViewSeq<D> {
    fn for_each(&self, f: &mut dyn FnMut(&dyn View<D>));

    fn for_each_mut(&mut self, f: &mut dyn FnMut(&mut dyn View<D>));
}

impl<D> ViewState<D> {
    pub fn new<T: 'static>(state: T) -> Self {
        Self { state: Box::new(state), _data: PhantomData }
    }

    pub fn state<T: 'static>(&self) -> &T {
        self.state.downcast_ref().unwrap()
    }

    pub fn state_mut<T: 'static>(&mut self) -> &mut T {
        self.state.downcast_mut().unwrap()
    }
}

impl<D> View<D> for () {
    fn init(&self, _cx: &mut ViewCx) -> ViewState<D> {
        ViewState::new(())
    }

    fn reinit(&self, _cx: &mut ViewCx, _state: &mut ViewState<D>) {}

    fn event(
        &mut self,
        _cx: &mut ViewCx,
        _state: &mut ViewState<D>,
        _event: Event,
        _data: &mut D,
    ) -> Option<()> {
        None
    }
}

impl<D> ViewSeq<D> for () {
    fn for_each(&self, _f: &mut dyn FnMut(&dyn View<D>)) {}

    fn for_each_mut(&mut self, _f: &mut dyn FnMut(&mut dyn View<D>)) {}
}

macro_rules! impl_view_seq {
    ($($t:ident)*) => {
        #[allow(non_snake_case)]
        impl<D, $($t),*> ViewSeq<D> for ($($t),*)
        where
            $($t: View<D>),*
        {
            fn for_each(&self, f: &mut dyn FnMut(&dyn View<D>)) {
                let ($($t),*) = self;

                $(f($t);)*
            }

            fn for_each_mut(&mut self, f: &mut dyn FnMut(&mut dyn View<D>)) {
                let ($($t),*) = self;

                $(f($t);)*
            }
        }
    };
}

impl_view_seq!(T0 T1);
impl_view_seq!(T0 T1 T2);
impl_view_seq!(T0 T1 T2 T3);
impl_view_seq!(T0 T1 T2 T3 T4);
impl_view_seq!(T0 T1 T2 T3 T4 T5);
impl_view_seq!(T0 T1 T2 T3 T4 T5 T6);
impl_view_seq!(T0 T1 T2 T3 T4 T5 T6 T7);
impl_view_seq!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
impl_view_seq!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_view_seq!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_view_seq!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
