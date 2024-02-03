use crate::widget::Widget;

pub trait View {
    fn for_each(&self, f: impl FnMut(&dyn Widget)) {}

    fn for_each_mut(&mut self, f: impl FnMut(&mut dyn Widget)) {}
}

impl View for () {}
