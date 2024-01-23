use wgpu::{BindGroup, ComputePipeline};

use crate::backend::Backend;

pub struct RasterizePipeline {
    pipeline: ComputePipeline,
    bindings: BindGroup,
}

impl RasterizePipeline {
    pub fn new(backend: &Backend) -> Self {
        todo!()
    }
}
