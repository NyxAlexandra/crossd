#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::error::Error;

use view::View;

pub mod graphics;
pub mod signal;
pub mod view;
pub mod widget;

pub fn launch<V: View>(f: impl FnOnce() -> V) -> Result {
    todo!()
}

pub type Result = std::result::Result<(), Box<dyn Error>>;
