import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct RayCastShader {
    distances: array<vec4<f32>, 256>,
}

@group(0) @binding(2) var<uniform> settings: RayCastShader;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let index = u32(round(in.uv.x * 255.0));
    let distance = settings.distances[index].x;

    let val = select(0.0, 1.0, 1.0 / distance > abs((in.uv.y - 0.5) * 2.0));

    return vec4<f32>(
        val,
        val,
        val,
        1.0
    );
}