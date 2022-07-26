#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

struct LineMaterial {
    start: vec3<f32>,
    end: vec3<f32>,
    width: f32,
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: LineMaterial;


struct Vertex {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex, @builtin(vertex_index) instance_id: u32 ) -> VertexOutput {
    var out: VertexOutput;
    var position = vertex.position;

    let dir = material.end - material.start;
    let normal = normalize(vec3(-dir.y, dir.x, 0.0));
    
    // TODO: Optimize this (if/else)
    if instance_id < u32(2) {
        // start

        position = material.start;
        if instance_id == u32(0) {
            position -= normal * material.width;
        } else {
            position += normal * material.width;;
        }
    } else {
        // end

        position = material.end;
        if instance_id == u32(2) {
            position += normal * material.width;
        } else {
            position -= normal * material.width;;
        }
    }

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));
    return out;
}


@fragment
fn fragment(
    // #import bevy_pbr::mesh_vertex_output
    input: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0);
}