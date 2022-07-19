fn disk(uv: vec2<f32>, radius: f32) -> f32 {
    let diameter = radius * 2.0;
    let dist: vec2<f32> = uv - vec2<f32>(0.5);
	let result = 1.0 - smoothstep(diameter - (diameter * 0.0041), diameter + (diameter * 0.0041), dot(dist, dist) * 4.0);
    return result;
}

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let color = vec4<f32>(disk(uv, 0.47));
    return color;
}