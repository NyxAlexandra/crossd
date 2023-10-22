use std::marker::PhantomData;
use std::mem;

use bytemuck::Pod;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

/// Storage for a single item on the gpu.
pub struct GpuStore<T> {
    buffer: Buffer,
    _ty: PhantomData<T>,
}

impl<T: Pod> GpuStore<T> {
    pub fn new(device: &Device, usage: BufferUsages, label: Option<&str>) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label,
            size: mem::size_of::<T>() as _,
            usage,
            mapped_at_creation: false,
        });

        Self { buffer, _ty: PhantomData }
    }

    /// Initialize a new store with a value.
    pub fn init(
        device: &Device,
        usage: BufferUsages,
        label: Option<&str>,
        item: T,
    ) -> Self {
        let contents = bytemuck::bytes_of(&item);
        let buffer =
            device.create_buffer_init(&BufferInitDescriptor { label, contents, usage });

        Self { buffer, _ty: PhantomData }
    }

    /// Write the new value to the store.
    pub fn write(&self, queue: &Queue, item: T) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&item));
    }

    /// The internal buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}
