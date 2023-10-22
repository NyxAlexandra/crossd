use std::convert::Infallible;

use crate::geometry::Size2;
use crate::{Canvas, Target};

impl Target for Canvas {
    type Error = Infallible;

    fn size(&self) -> Size2<u32> {
        self.size
    }

    fn present(&self) -> Result<(), Infallible> {
        todo!()
    }
}
