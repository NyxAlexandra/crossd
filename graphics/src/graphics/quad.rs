use std::mem;

use bytemuck::{Pod, Zeroable};
use crossd_math::Mat4;
use wgpu::{
    BindGroup,
    BindGroupDescriptor,
    BindGroupEntry,
    BindGroupLayout,
    BindGroupLayoutDescriptor,
    BindGroupLayoutEntry,
    BindingType,
    BlendComponent,
    BlendFactor,
    BlendOperation,
    BlendState,
    BufferBindingType,
    BufferSize,
    BufferUsages,
    ColorTargetState,
    ColorWrites,
    FragmentState,
    FrontFace,
    IndexFormat,
    MultisampleState,
    PipelineLayoutDescriptor,
    PrimitiveState,
    PrimitiveTopology,
    RenderPass,
    RenderPipeline,
    RenderPipelineDescriptor,
    ShaderModuleDescriptor,
    ShaderSource,
    ShaderStages,
    VertexAttribute,
    VertexBufferLayout,
    VertexFormat,
    VertexState,
    VertexStepMode,
};

use crate::backend::Backend;
use crate::geometry::Rect;
use crate::primitive::{Quad, Vertex};
use crate::utils::gpu_buf::GpuBuf;
use crate::utils::gpu_store::GpuStore;
use crate::{Draw, Frame, Target};

pub struct QuadPipeline {
    pipeline: RenderPipeline,
    /// [`QuadUniforms`] layout.
    layout: BindGroupLayout,

    vertices: GpuBuf<Vertex>,
    indices: GpuBuf<u16>,

    /// Layers of quads.
    layers: Vec<QuadLayer>,
    /// The current layer.
    current: usize,
}

struct QuadLayer {
    /// Uniform bind group.
    bgroup: BindGroup,
    /// Uniform storage buffer.
    uniforms: GpuStore<QuadUniforms>,
    /// Quads in this layer.
    quads: GpuBuf<Quad>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct QuadUniforms {
    trans: Mat4,
    scale: f32,
    /// Uniforms must be aligned to their largest member.
    _padding: [f32; 3],
}

/// Shader source.
const SOURCE: &str = include_str!("../shader/quad.wgsl");

/// Initial [`QuadLayer`] quad buffer capacity.
const INITIAL: usize = 2000;
/// Connections between the vertices, forming a rectangle.
const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];
/// Vertices in the shape of a square.
const VERTICES: [Vertex; 4] = [
    Vertex::new(0.0, 1.0),
    Vertex::new(1.0, 0.0),
    Vertex::new(1.0, 1.0),
    Vertex::new(0.0, 1.0),
];

impl QuadPipeline {
    pub fn new(backend: &Backend) -> Self {
        let device = backend.device();

        let layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("QuadPipeline.layout"),
                // `@group(0)`,
                entries: &[BindGroupLayoutEntry {
                    // ...`@binding(0)`
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            mem::size_of::<QuadUniforms>() as _
                        ),
                    },
                    // none indicates that this is not an array
                    count: None,
                }],
            });

        let pipeline = {
            let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("QuadPipeline.pipeline:layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });

            let module = device.create_shader_module(ShaderModuleDescriptor {
                label: Some("QuadPipeline.pipeline:module"),
                source: ShaderSource::Wgsl(SOURCE.into()),
            });

            let vertex = VertexState {
                module: &module,
                entry_point: "vert",
                buffers: &[VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as _,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        // layout for `Vertex` (`Point2`)
                        format: VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            };
            let primitive = PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                front_face: FrontFace::Cw,
                ..Default::default()
            };
            let multisample = MultisampleState {
                // non-multisampled -> `1`
                count: 1,
                // `!0` -> all samples
                mask: !0,
                alpha_to_coverage_enabled: false,
            };
            let targets = &[Some(ColorTargetState {
                format: backend.format(),
                blend: Some(BlendState {
                    color: BlendComponent {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                }),
                write_mask: ColorWrites::ALL,
            })];

            device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("QuadPipeline.pipeline"),
                layout: Some(&layout),
                vertex,
                primitive,
                depth_stencil: None,
                multisample,
                fragment: Some(FragmentState {
                    module: &module,
                    entry_point: "frag",
                    targets,
                }),
                multiview: None,
            })
        };

        let vertices = GpuBuf::init(
            device,
            BufferUsages::VERTEX,
            Some("QuadPipeline.vertices"),
            &VERTICES,
        );
        let indices = GpuBuf::init(
            device,
            BufferUsages::INDEX,
            Some("QuadPipeline.indices"),
            &INDICES,
        );

        Self { pipeline, layout, vertices, indices, layers: Vec::new(), current: 0 }
    }

    /// Prepare the current layer.
    pub fn prepare(
        &mut self,
        backend: &Backend,
        trans: Mat4,
        scale: f32,
        quads: &[Quad],
    ) {
        if self.layers.len() <= self.current {
            self.layers.push(QuadLayer::new(backend, &self.layout));
        }

        self.layers[self.current].prepare(backend, trans, scale, quads);
        self.current += 1;
    }

    /// Render the requested layer.
    pub fn render<'pass>(
        &'pass self,
        layer: usize,
        bounds: Rect<u32>,
        rpass: &mut RenderPass<'pass>,
    ) {
        rpass.set_pipeline(&self.pipeline);

        if let Some(layer) = self.layers.get(layer) {
            rpass.set_scissor_rect(
                bounds.loc.x,
                bounds.loc.y,
                bounds.size.w,
                bounds.size.h,
            );
            rpass.set_index_buffer(self.indices.slice(..), IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, self.vertices.slice(..));

            // use layer uniforms
            rpass.set_bind_group(0, &layer.bgroup, &[]);
            rpass.set_vertex_buffer(1, layer.quads.slice(..));

            // draw all quads in the layer
            rpass.draw_indexed(0..INDICES.len() as _, 0, 0..layer.quads.len() as _);
        }
    }
}

impl QuadLayer {
    fn new(backend: &Backend, layout: &BindGroupLayout) -> Self {
        let device = backend.device();

        let uniforms = GpuStore::new(
            device,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            Some("QuadLayer.uniforms"),
        );
        let bgroup = device.create_bind_group(&BindGroupDescriptor {
            label: Some("QuadLayer.bgroup"),
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniforms.buffer().as_entire_binding(),
            }],
        });

        let quads = GpuBuf::new(
            device,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            Some("QuadLayer.quads"),
            INITIAL,
        );

        Self { bgroup, uniforms, quads }
    }

    fn prepare(&mut self, backend: &Backend, trans: Mat4, scale: f32, quads: &[Quad]) {
        let device = backend.device();
        let queue = backend.queue();

        self.uniforms.write(queue, QuadUniforms::new(trans, scale));
        self.quads.write(device, queue, quads);
    }
}

impl QuadUniforms {
    /// Create uniforms with given values.
    const fn new(trans: Mat4, scale: f32) -> Self {
        Self { trans, scale, _padding: [0.0; 3] }
    }

    /// Uniform using the identity matrix for the transform.
    const fn identity() -> Self {
        Self::new(Mat4::IDENTITY, 1.0)
    }
}

impl Default for QuadUniforms {
    fn default() -> Self {
        Self::new(Mat4::default(), 1.0)
    }
}

impl Draw for Quad {
    fn draw(frame: &mut Frame<'_, impl Target>, quad: Self) {
        frame.context.with_mut(|state| state.scene.current_mut().add_quad(quad))
    }
}
