import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct RayCastShader {
    rays: array<vec4<f32>, 256>,
}

@group(0) @binding(2) var<uniform> settings: RayCastShader;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let index = u32(round(in.uv.x * 255.0));
    let distance = settings.rays[index].x;

    let actual_wall_height = 1.0;
    let viewport_size = vec2<f32>(textureDimensions(screen_texture));
    let screen_wall_height = actual_wall_height * viewport_size.y / viewport_size.x;

    let val = select(0.0, 1.0, 1.0 / distance > abs((in.uv.y - 0.5) * 6.0 * screen_wall_height)) / distance;

    let color = vec3(1.0, settings.rays[index].y, 1.0) * val;

    return vec4<f32>(
        color,
        1.0
    );
}