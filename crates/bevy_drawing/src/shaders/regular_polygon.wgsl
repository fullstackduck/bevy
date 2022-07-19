// TODO: change away from length to avoid sqrt
fn regular_polygon(uv: vec2<f32>, radius: f32) -> f32 {
    var uv = uv;
    uv = uv * 2.0 - 1.0;

    let TWO_PI = 6.28318530718;

    let N = 3;
    if (N == 3) {
        uv.y = uv.y - 0.234;
    }

    // Angle and radius from the current pixel
    let a = atan2(uv.x, uv.y);
    let r = TWO_PI / f32(N);

    // Shaping function that modulate the distance
    var d = cos(floor(0.5 + a / r) * r - a) * length(uv);
	if (N > 3) {
        d = d * 0.55;
    } else {
        d = d * 0.808;
    }
    return 1.0 - smoothstep(0.4, 0.4015, d);
}

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let color = vec4<f32>(regular_polygon(uv, 0.47));
    return color;
}