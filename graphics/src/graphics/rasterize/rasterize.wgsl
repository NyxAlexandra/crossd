struct Block {
    idx: u32,
    len: u32,
}

struct Primitive {
    path: Slice,
    style: u32,
}

struct Style {
    flags: StyleFlags,
    miter: f32,
    width: f32,
}

struct StyleFlags {
    bits: u32,
}

struct Slice {
    idx: u32,
    len: u32,
}

@group(0) @binding(0)
var<storage> primitives: array<Primitive>;
@group(0) @binding(1)
var<storage> paths: array<vec2f>;
@group(0) @binding(2)
var<storage> styles: array<Style>;

@group(1) @binding(0)
var<storage> output: texture_storage_2d<rgba8unorm, write>;

// ---

@compute @workgroup_size(Block_WIDTH, Block_HEIGHT)
fn comp(
    @builtin(global_invocation_id)
    giid: vec3u,
    @builtin(local_invocation_id)
    liid: vec3u,
    @builtin(workgroup_id)
    wgid: vec3u,
) {
    let block = blocks[wgid.y * uniforms.row_width + wgid.x];

    let pixel = giid.xy;

    var rgba: vec4f;
    var area = 0.0;

    for (
        var prim_idx = block.idx;
        prim_idx < arrayLength(&primitives);
        prim_idx += 1
    ) {
        let prim = primitives[prim_idx];
        
        if StyleFlags_is_fill(prim.flags) {
            fill_path(path, pixel, prim, &rgba, &area);
        } else {
            fill_stroke(path, pixel, prim, &rgba, &area);
        }
    }

    // calculate pixels from area and winding

    textureStore(output, pixel, rgba);
}

fn fill_path(
    path: Path,
    pixel: vec2f,
    prim: Primitive,
    rgba: ptr<function, vec4f>,
    area: ptr<function, f32>,
) {}

fn fill_stroke(
    path: Path,
    pixel: vec2f,
    prim: Primitive,
    rgba: ptr<function, vec4f>,
    area: ptr<function, f32>,
) {
    // TODO
}

// ---

let Block_WIDTH: u32 = 16;
let Block_HEIGHT: u32 = 16;

let FillMode_EvenOdd: u32 = 2;
let FillMode_NonZero: u32 = 3;

fn StyleFlags_is_fill(self: StyleFlags) -> bool {
    return (self.bits & 1) == 0;
}

fn StyleFlags_fill_mode(self: StyleFlags) -> u32 {
    return self.bits & 2;
}
