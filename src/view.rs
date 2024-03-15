use std::any::Any;
use std::marker::PhantomData;

use crossgl::scene::Scene;

use crate::layout::{Bounds, Layout};
use crate::math::{Rect, Size2};

pub trait View<D> {
    fn init(&self) -> ViewState<D>;

    fn reinit(&self, state: &mut ViewState<D>);

    fn update(&mut self, state: &mut ViewState<D>, event: Event, data: &mut D);

    fn layout(&mut self, bounds: Bounds, data: &D) -> Size2<u32>;

    fn paint(&mut self, geo: Rect<u32>, data: &D, scene: &mut Scene);
}

pub trait ViewSeq<D> {
    fn for_each(&self, f: &mut dyn FnMut(&dyn View<D>));

    fn for_each_mut(&mut self, f: &mut dyn FnMut(&mut dyn View<D>));
}

pub struct ViewState<D> {
    state: Box<dyn Any>,
    _data: PhantomData<D>,
}

pub enum Event {}

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
    fn init(&self) -> ViewState<D> {
        ViewState::new(())
    }

    fn reinit(&self, _state: &mut ViewState<D>) {}

    fn update(&mut self, _state: &mut ViewState<D>, _eventt: Event, _data: &mut D) {}

    fn layout(&mut self, bounds: Bounds, _data: &D) -> Size2<u32> {
        Layout::default().resolve(bounds)
    }

    fn paint(&mut self, _geo: Rect<u32>, _data: &D, _scene: &mut Scene) {}
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

impl_view_seq!(V0 V1);
impl_view_seq!(V0 V1 V2);
impl_view_seq!(V0 V1 V2 V3);
impl_view_seq!(V0 V1 V2 V3 V4);
impl_view_seq!(V0 V1 V2 V3 V4 V5);
impl_view_seq!(V0 V1 V2 V3 V4 V5 V6);
impl_view_seq!(V0 V1 V2 V3 V4 V5 V6 V7);
impl_view_seq!(V0 V1 V2 V3 V4 V5 V6 V7 V8);
impl_view_seq!(V0 V1 V2 V3 V4 V5 V6 V7 V8 V9);
impl_view_seq!(V0 V1 V2 V3 V4 V5 V6 V7 V8 V9 V10);
impl_view_seq!(V0 V1 V2 V3 V4 V5 V6 V7 V8 V9 V10 V11);
