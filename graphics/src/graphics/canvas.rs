use crate::geometry::Size2;
use crate::{Canvas, Target};

impl Target for Canvas {
    fn size(&self) -> Size2<u32> {
        self.size
    }

    fn present(&self) {
        todo!()
    }
}
