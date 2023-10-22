//! A buffer of items on the GPU, [`GpuBuf`].

use std::marker::PhantomData;
use std::mem;
use std::ops::RangeBounds;

use bytemuck::Pod;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    Buffer,
    BufferAddress,
    BufferDescriptor,
    BufferSlice,
    BufferUsages,
    Device,
    Queue,
};

pub struct GpuBuf<T> {
    buffer: Buffer,
    /// Label for the buffer.
    ///
    /// Used when reallocating it.
    label: Option<String>,
    /// Capacity of the buffer.
    cap: usize,
    /// Amount of items.
    len: usize,

    _ty: PhantomData<T>,
}

impl<T: Pod> GpuBuf<T> {
    /// Create a new uninitialized buffer on the GPU.
    pub fn new(
        device: &Device,
        usage: BufferUsages,
        label: Option<&str>,
        cap: usize,
    ) -> Self {
        let size = next_copy_size::<T>(cap);
        let buffer = device.create_buffer(&BufferDescriptor {
            label,
            size,
            usage,
            mapped_at_creation: false,
        });

        let label = label.map(ToOwned::to_owned);

        Self { label, buffer, cap, len: 0, _ty: PhantomData }
    }

    /// Initialize a new buffer on the GPU.
    pub fn init(
        device: &Device,
        usage: BufferUsages,
        label: Option<&str>,
        items: &[T],
    ) -> Self {
        let len = items.len();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(items),
            usage,
        });

        let label = label.map(ToOwned::to_owned);

        Self { label, buffer, cap: len, len, _ty: PhantomData }
    }

    /// Overwrite existing buffer contents with new content.
    ///
    /// Returns `true` if a re-allocation occured.
    pub fn write(&mut self, device: &Device, queue: &Queue, items: &[T]) -> bool {
        if items.len() > self.cap {
            self.resize(device, next_copy_size::<T>(items.len()) as _);

            true
        } else {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(items));

            false
        }
    }

    /// The amount of items in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// The capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// A slice into the [buffer on the gpu](Buffer).
    pub fn slice(&self, bounds: impl RangeBounds<BufferAddress>) -> BufferSlice<'_> {
        self.buffer.slice(bounds)
    }

    /// Update the value of the item at the provided index.
    pub fn set(&mut self, queue: &Queue, index: usize, item: T) {
        todo!()
    }

    /// Reallocate the buffer, returning `true`.
    ///
    /// This clears all existing content.
    pub fn resize(&mut self, device: &Device, cap: usize) {
        let new = (cap * mem::size_of::<T>()) as BufferAddress;

        if self.buffer.size() < new {
            self.buffer = device.create_buffer(&BufferDescriptor {
                label: self.label.as_deref(),
                size: new,
                usage: self.buffer.usage(),
                mapped_at_creation: false,
            });
            self.len = 0;
            self.cap = new as _;
        }
    }
}

// from <https://github.com/iced-rs/iced/blob/0770e7eaf842021a4b15b00e1b81ba10dd9b8140/wgpu/src/buffer.rs#L99C1-L99C1>
fn next_copy_size<T>(len: usize) -> u64 {
    let align_mask = wgpu::COPY_BUFFER_ALIGNMENT - 1;

    (((mem::size_of::<T>() * len).next_power_of_two() as u64 + align_mask) & !align_mask)
        .max(wgpu::COPY_BUFFER_ALIGNMENT)
}
