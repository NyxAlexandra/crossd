use std::marker::PhantomData;
use std::mem;

use bytemuck::Pod;
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{
    BufferUsages,
    Device,
    Extent3d,
    ImageDataLayout,
    Instance,
    Queue,
    Surface,
    TextureFormat,
    TextureUsages,
};

use crate::math::{Point2, Rect, Size2};
use crate::scene::Image;

/// Core Wgpu resources packed into a single type for convenience.
pub struct Context {
    instance: Instance,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
}

/// A handle to a buffer on the GPU.
#[derive(Debug)]
pub struct Buffer<T> {
    inner: wgpu::Buffer,
    _ty: PhantomData<T>,
}

/// [`Buffer`] builder interface.
#[derive(Debug)]
pub struct BufferBuilder {
    pub usage: BufferUsages,
}

/// Utility for creating dynamic buffers.
#[derive(Debug)]
pub struct BufferVec<T> {
    vec: Vec<T>,
    buffer: Buffer<T>,
    len: usize,
}

/// A handle to image data on the GPU.
#[derive(Debug)]
pub struct Texture {
    inner: wgpu::Texture,
    size: Size2<u32>,
}

/// [`Texture`] builder interface.
///
/// As all fields are public, this type can also be created with struct syntax.
#[derive(Debug)]
pub struct TextureBuilder {
    pub usage: TextureUsages,
}

impl Texture {
    /// Create a new empty texture.
    pub fn new(
        cx: &Context,
        usage: TextureUsages,
        format: TextureFormat,
        size: impl Into<Size2<u32>>,
    ) -> Self {
        Self::builder().usage(usage).build(cx, format, size)
    }

    /// Shorthand for [`TextureBuilder::new`].
    pub fn builder() -> TextureBuilder {
        TextureBuilder::new()
    }

    pub fn size(&self) -> Size2<u32> {
        self.size
    }

    /// Inner GPU texture handle.
    pub fn inner(&self) -> &wgpu::Texture {
        &self.inner
    }

    /// Write an image onto this texture at the given coordinates.
    pub fn write(
        &self,
        cx: &Context,
        rect: impl Into<Rect<u32>>,
        image: &Image,
    ) -> Result<(), TextureWriteError> {
        cx.queue.write_texture(
            self.inner.as_image_copy(),
            &image.data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: image
                    .format
                    .block_copy_size(None)
                    .map(|size| size * image.size.w),
                rows_per_image: None,
            },
            Extent3d {
                width: image.size.w,
                height: image.size.h,
                depth_or_array_layers: 1,
            },
        );

        Ok(())
    }
}

#[derive(Error, Debug, Clone, Copy)]
pub enum TextureWriteError {
    #[error("point {point:?} out of texture bounds {bounds:?}")]
    PointOutOfBounds { point: Point2<u32>, bounds: Rect<u32> },
}

impl TextureBuilder {
    /// Create a new builder.
    ///
    /// No GPU resources are created until the builder is destroyed.
    pub fn new() -> Self {
        let usage = TextureUsages::all();

        Self { usage }
    }

    /// Set which usages are possible for this texture.
    pub fn usage(self, usage: TextureUsages) -> Self {
        Self { usage, ..self }
    }

    /// Create the specified blank texture with the given width and height.
    pub fn build(
        self,
        cx: &Context,
        format: TextureFormat,
        size: impl Into<Size2<u32>>,
    ) -> Texture {
        todo!()
    }

    /// Create a new texture from an image.
    pub fn init(self, cx: &Context, image: &Image) -> Texture {
        todo!()
    }
}

impl Default for TextureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Pod> Buffer<T> {
    /// Create a new buffer.
    pub fn new(cx: &Context, usage: BufferUsages, capacity: usize) -> Self {
        Self::builder().usage(usage).build(cx, capacity)
    }

    /// Shorthand for [`BufferBuilder::new`].
    pub fn builder() -> BufferBuilder {
        BufferBuilder::new()
    }

    /// Capacity of the GPU buffer.
    pub fn capacity(&self) -> usize {
        self.size() / mem::size_of::<T>()
    }

    /// Size of the buffer in bytes.
    pub fn size(&self) -> usize {
        self.inner.size() as _
    }

    /// What this buffer can be used for.
    pub fn usage(&self) -> BufferUsages {
        self.inner.usage()
    }

    /// Handle to the buffer on the GPU.
    pub fn inner(&self) -> &wgpu::Buffer {
        &self.inner
    }

    /// Cast the buffer to be interpreted as containing a different type.
    ///
    /// This method is safe, as both `T` and `U` implement [`Pod`].
    pub fn cast<U: Pod>(self) -> Buffer<U> {
        let Self { inner: buffer, .. } = self;

        Buffer { inner: buffer, _ty: PhantomData }
    }

    /// Write a slice to the buffer at the provided index.
    pub fn write(
        &mut self,
        cx: &Context,
        index: usize,
        slice: &[T],
    ) -> Result<(), BufferWriteError> {
        let length = slice.len();
        let capacity = self.capacity();

        if index >= self.capacity() {
            return Err(BufferWriteError::IndexOutOfBounds { index, capacity });
        }

        if index + length > capacity {
            return Err(BufferWriteError::Overflow { index, length, capacity });
        }

        cx.queue.write_buffer(
            &self.inner,
            (index * mem::size_of::<T>()) as _,
            bytemuck::cast_slice(slice),
        );

        Ok(())
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BufferWriteError {
    #[error(
        "provided index {index} is out of bounds for buffer with capacity {capacity}"
    )]
    IndexOutOfBounds { index: usize, capacity: usize },
    #[error("new length {} ({index} + {length}) would overflow buffer with capacity {capacity}", index + length)]
    Overflow { index: usize, length: usize, capacity: usize },
}

impl BufferBuilder {
    /// Create a new builder.
    ///
    /// no GPU resources are created until the builder is consumed.
    pub fn new() -> Self {
        Self { usage: BufferUsages::all() }
    }

    /// Set what this buffer can be used for.
    pub fn usage(self, usage: BufferUsages) -> Self {
        Self { usage, ..self }
    }

    /// Create an empty buffer with the given capacity.
    pub fn build<T: Pod>(self, cx: &Context, capacity: usize) -> Buffer<T> {
        let Self { usage: usages } = self;

        let buffer = cx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (mem::size_of::<T>() * capacity) as _,
            usage: usages,
            mapped_at_creation: false,
        });

        Buffer { inner: buffer, _ty: PhantomData }
    }

    /// Initialize a new buffer with the contents of a slice.
    pub fn init<T: Pod>(self, cx: &Context, slice: &[T]) -> Buffer<T> {
        let Self { usage } = self;

        let buffer = cx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(slice),
            usage,
        });

        Buffer { inner: buffer, _ty: PhantomData }
    }
}

impl Default for BufferBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Pod> BufferVec<T> {
    pub fn new(cx: &Context, buffer: BufferBuilder, capacity: usize) -> Self {
        let vec = Vec::with_capacity(capacity);
        let buffer = buffer.build(cx, capacity);
        let len = 0;

        Self { vec, buffer, len }
    }

    pub fn init(cx: &Context, buffer: BufferBuilder, items: &[T]) -> Self {
        let vec = items.to_vec();
        let buffer = buffer.init(cx, &vec);
        let len = vec.len();

        Self { vec, buffer, len }
    }

    pub fn vec(&self) -> &Vec<T> {
        &self.vec
    }

    pub fn vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.vec
    }

    pub fn buffer(&self) -> &Buffer<T> {
        &self.buffer
    }
}
