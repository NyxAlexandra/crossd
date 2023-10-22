use wgpu::{CommandEncoder, RenderPass};

use crate::{Draw, Frame, Graphics, Target};

impl<'frame, T: Target> Frame<'frame, T> {
    fn new(
        graphics: &Graphics,
        encoder: CommandEncoder,
        rpass: RenderPass<'frame>,
        target: &'frame T,
    ) -> Self {
        let backend = graphics.backend.clone();
        let context = graphics.context.clone();

        Self { backend, context, rpass, encoder, target }
    }

    /// Draw an item.
    pub fn draw(&mut self, item: impl Draw) {
        Draw::draw(self, item)
    }
}
