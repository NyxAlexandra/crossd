use std::error::Error;

use crossd_graphics::{Frame, Graphics, Surface};
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[allow(unused)]
fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let graphics = Graphics::new()?;

    let eloop = EventLoop::new();
    let window = WindowBuilder::new().build(&eloop)?;
    let target = Surface::new(&graphics, window);

    // ugly... maybe I should get over it, or maybe I should rework my API
    let mut frame: Option<Frame<Surface<Window>>> = None;

    eloop.run(move |event, _, ctrl| match event {
        Event::WindowEvent { event, .. } => todo!(),
        Event::MainEventsCleared => todo!(),
        Event::RedrawRequested(_) => todo!(),
        _ => {},
    })
}
