struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

struct PlainParams {
    phase: f32,
    frequency: f32,
};


@group(0) @binding(0) // 10.
var<uniform> params: PlainParams;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), // bottom left
        vec2<f32>(1.0, -1.0),  // bottom right
        vec2<f32>(-1.0, 1.0),  // top left
        vec2<f32>(-1.0, 1.0),  // top left
        vec2<f32>(1.0, -1.0),  // bottom right
        vec2<f32>(1.0, 1.0)    // top right
    );
    let pos = positions[in_vertex_index];

    return VertexOutput(
        vec4<f32>(pos, 0., 1.),
        pos
    );

}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normalized = (in.coord + vec2<f32>(1., 1.)) / 2.;
    var alpha = sin(params.frequency * normalized.x + params.phase);
    return vec4<f32>(1.0 * alpha, 1.0 * alpha, 1.0 * alpha, 1.0);
}
