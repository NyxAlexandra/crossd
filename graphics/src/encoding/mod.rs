use bytemuck::{Pod, Zeroable};
use crossd_math::{Point2, Size2};
use wgpu::{BindGroup, BindGroupLayout};

use crate::backend::Backend;
use crate::scene::{Color, Scene};
use crate::utils::gpu_vec::GpuVec;

pub struct Encoding {
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,

    elements: GpuVec<Element>,
    segments: GpuVec<Point2>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Element {
    path: Slice,
    color: Color,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Slice {
    idx: u32,
    len: u32,
}

impl Encoding {
    pub fn new(backend: &Backend) -> Self {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn prepare(&mut self, backend: &Backend, scene: &Scene, size: Size2) {
        // (move || {
        //     let tolerance = todo!();
        //     let Path { verbs, points } = path;

        //     let mut verbs = verbs.into_iter();
        //     let mut points = points.into_iter();

        //     let mut loc = None;

        //     while let Some(verb) = verbs.next() {
        //         match verb {
        //             PathVerb::Move => {
        //                 loc = points.next();
        //             },
        //             PathVerb::Line => {
        //                 self.segments.extend([loc?, points.next()?]);
        //             },
        //             PathVerb::Quad => {
        //                 todo!("flatten")
        //             },
        //             PathVerb::Cubic => {
        //                 todo!("flatten")
        //             },
        //             PathVerb::Close => {
        //                 todo!("flatten")
        //             },
        //         }
        //     }

        //     Some(())
        // })()
        // .unwrap()
    }
}
