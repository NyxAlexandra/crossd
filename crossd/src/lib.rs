use std::error::Error;

use view::View;

pub mod graphics;
pub mod layout;
pub mod view;
pub mod widget;

pub enum Event {}

pub fn launch<V, D>(data: D, f: impl FnOnce(&mut D) -> V) -> Result
where
    V: View<D>,
{
    todo!()
}

pub type Result = std::result::Result<(), Box<dyn Error>>;
