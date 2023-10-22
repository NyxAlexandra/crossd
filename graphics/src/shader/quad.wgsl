struct Uniforms {
    trans: Mat4x4<f32>,
    scale: f32,
}

struct VertexInput {
    @location(0)
    loc: vec2<f32>,
    @location(1)
    color: vec4<f32>,
    @location(2)
    scale: vec2<f32>,
}

struct VertexOutput {
    @builtin(position)
    position: vec4<f32>,
    @location(0)
    color: vec4<f32>,
    @location(1)
    loc: vec2<f32>,
    @location(2)
    scale: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vert(in: VertexInput) -> VertexOutput {
    var loc: vec2<f32> = in.pos * uniforms.scale;
    var scale: vec2<f32> = in.scale * uniforms.scale;

    var trans: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, scale.y + 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(loc - vec2<f32>(0.5, 0.5), 0.0, 1.0)
    );

    var out: VertexOutput;

    out.position =
        uniforms.trans
        * trans
        * vec4<f32>(in.loc, 0.0, 1.0);
    out.color = in.color;
    out.loc = loc;
    out.scale = scale;

    return out;
}

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    var out: vec4<f32> = in.color;

    return out;
}