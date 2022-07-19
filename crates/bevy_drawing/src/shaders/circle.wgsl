// #import bevy_pbr::mesh_view_bind_group
// #import bevy_pbr::mesh_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

// [[group(1), binding(0)]]
// var<uniform> mesh: Mesh;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let model = mesh.model;
    let world_position = model * vec4<f32>(vertex.position, 1.0);
    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    out.uv = vertex.uv;
    return out;
}

// change away for length to avoid sqrt
fn circle(uv: vec2<f32>, radius: f32) -> f32 {
    let l = length(uv - 0.5);
    let result = smoothStep(4.0 / view.height, 0.0, abs(l - radius));
    return result;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let color = vec4<f32>(circle(in.uv, 0.47));
    return color;
}
