/// Pipeline configuration.
@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;

/// Track allocation needs.
@group(0)
@binding(1)
var<storage, read_write> alloc: Alloc;

// flatten - flatten paths into line segments
// 
// accuracy of flattening is controlled by tolerance, which is derived
// from the size of the window.

var<storage, read_write> line_segments: array<vec2f>;

// drawtag_reduce - turn draw operation stream into monads

let DRAWTAG_REDUCE: u32 = 256;

var<storage, read_write> draw_monoids: array<DrawMonoid>;
var<workgroup> drawtag_scratch: array<DrawMonoid, DRAWTAG_REDUCE>;

struct DrawMonoid {
    path_count: u32,
    clip_count: u32,
    scene_idx: u32,
    data_offset: u32,
}

@compute @workgroup_size(DRAWTAG_REDUCE)
fn drawtag_reduce(
    @builtin(global_invocation_id) giid: vec3u,
    @builtin(local_invocation_id) liid: vec3u,
) {}

// bin - separate line segments into "bins"

// rasterize - paint each pixel
