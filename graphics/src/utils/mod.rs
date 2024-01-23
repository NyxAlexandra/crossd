use std::mem;

pub mod gpu_buf;
pub mod gpu_store;
pub mod gpu_vec;
pub mod uniform_store;

// from <https://github.com/iced-rs/iced/blob/0770e7eaf842021a4b15b00e1b81ba10dd9b8140/wgpu/src/buffer.rs#L99C1-L99C1>
fn next_copy_size<T>(len: usize) -> u64 {
    let align_mask = wgpu::COPY_BUFFER_ALIGNMENT - 1;

    (((mem::size_of::<T>() * len).next_power_of_two() as u64 + align_mask) & !align_mask)
        .max(wgpu::COPY_BUFFER_ALIGNMENT)
}
