use std::mem;
use std::slice::{Iter, IterMut};

use bytemuck::Pod;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device};

use super::next_copy_size;
use crate::backend::Backend;

pub struct GpuVec<T> {
    local: Vec<T>,

    remote: Buffer,
    len: usize,

    label: Option<String>,
}

impl<T: Pod> GpuVec<T> {
    /// Create a new zeroad buffer.
    pub fn new(
        device: &Device,
        usage: BufferUsages,
        label: Option<&str>,
        cap: usize,
    ) -> Self {
        let size = next_copy_size::<T>(cap);
        let remote = device.create_buffer(&BufferDescriptor {
            label,
            size,
            usage,
            mapped_at_creation: false,
        });
        let local = Vec::with_capacity(cap);
        let label = label.map(ToOwned::to_owned);

        Self { local, remote, len: 0, label }
    }

    /// Initialize a new buffer with given data.
    pub fn init(
        device: &Device,
        usage: BufferUsages,
        label: Option<&str>,
        items: &[T],
    ) -> Self {
        let len = items.len();
        let remote = device.create_buffer_init(&BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(items),
            usage,
        });
        let local = items.to_vec();

        let label = label.map(ToOwned::to_owned);

        Self { local, remote, len, label }
    }

    /// Amount of items current stored in the remote buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Amount of `T` the remote buffer can store.
    pub fn capacity(&self) -> usize {
        self.remote.size() as usize / mem::size_of::<T>()
    }

    pub fn local(&self) -> &[T] {
        &self.local
    }

    pub fn local_mut(&mut self) -> &mut [T] {
        &mut self.local
    }

    pub fn remote(&self) -> &Buffer {
        &self.remote
    }

    pub fn push(&mut self, item: T) {
        self.local.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.local.pop()
    }

    pub fn insert(&mut self, idx: usize, item: T) {
        self.local.insert(idx, item)
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if self.local.get(idx).is_some() {
            Some(self.local.remove(idx))
        } else {
            None
        }
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = T>) {
        self.local.extend(iter)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.local.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.local.iter_mut()
    }

    /// Returns `true` if reallocation occurred.
    pub fn sync(&mut self, backend: &Backend) -> bool {
        let realloc = if self.local.len() > self.capacity() {
            self.remote = backend.device().create_buffer(&BufferDescriptor {
                label: self.label.as_deref(),
                size: (self.local.len() * mem::size_of::<T>()) as _,
                usage: self.remote.usage(),
                mapped_at_creation: false,
            });
            self.len = self.local.len();

            true
        } else {
            false
        };

        backend.queue().write_buffer(&self.remote, 0, bytemuck::cast_slice(&self.local));

        realloc
    }
}
