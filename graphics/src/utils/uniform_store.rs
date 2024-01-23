use bytemuck::Pod;
use wgpu::{BindGroup, BufferUsages, Device, Queue};

use super::gpu_store::GpuStore;

pub struct UniformStore<T> {
    buffer: GpuStore<T>,
    bgroup: BindGroup,
}

impl<T: Pod> UniformStore<T> {
    pub fn new(device: &Device, bgroup: BindGroup, label: Option<&str>) -> Self {
        let buffer = GpuStore::new(device, BufferUsages::UNIFORM, label);

        Self { buffer, bgroup }
    }

    pub fn init(
        device: &Device,
        bgroup: BindGroup,
        label: Option<&str>,
        item: T,
    ) -> Self {
        let buffer = GpuStore::init(device, BufferUsages::UNIFORM, label, item);

        Self { buffer, bgroup }
    }

    pub fn write(&mut self, queue: &Queue, item: T) {
        self.buffer.write(queue, item)
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bgroup
    }

    pub fn set_bind_group(&mut self, bgroup: BindGroup) {
        self.bgroup = bgroup;
    }
}
