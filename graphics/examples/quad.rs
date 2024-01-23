use std::error::Error;

use crossd_graphics::graphics::surface::Surface;
use crossd_graphics::graphics::{Graphics, RenderTarget};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let mut graphics = Graphics::new()?;

    let eloop = EventLoop::new();
    let window = WindowBuilder::new().build(&eloop)?;
    let mut surface = Surface::new(window, &mut graphics)?;

    eloop.run(move |event, _, ctrl| {
        dispatch(event, ctrl, &mut graphics, &mut surface).unwrap();
    })
}

fn dispatch(
    event: Event<'_, ()>,
    ctrl: &mut ControlFlow,
    graphics: &mut Graphics,
    surface: &mut Surface<Window>,
) -> Result<(), Box<dyn Error>> {
    Ok(match event {
        Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => ctrl.set_exit(),
        Event::MainEventsCleared => {
            surface.prepare(graphics);
            surface.target().request_redraw();
        },
        Event::RedrawRequested(_) => {
            surface.render(graphics);
        },
        _ => {},
    })
}
