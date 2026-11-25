import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct RayCastShader {
    rays: array<vec4<f32>, 1024>,
}

@group(0) @binding(2) var<uniform> settings: RayCastShader;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let index = u32(round(in.uv.x * 1023.0));
    let distance = settings.rays[index].x;

    let actual_wall_height = 1.0;
    let viewport_size = vec2<f32>(textureDimensions(screen_texture));
    let screen_wall_height = actual_wall_height * viewport_size.y / viewport_size.x;

    let screen_eye_distance_signed = (in.uv.y - 0.5) * 2.0;

    let wall_eye_distance_signed = screen_eye_distance_signed * 3.0 * screen_wall_height * distance;
    let wall_eye_distance = abs(wall_eye_distance_signed);

    let val = select(0.0, 1.0, 1.0 > wall_eye_distance) / distance;

    let uv = vec2(settings.rays[index].y, (1.0 - wall_eye_distance_signed) * 0.5);

    let bricks = select(
        0.5, 1.0, uv.y % 0.1 < 0.025 || 
        (uv.x % 0.2 < 0.025 && uv.y % 0.2 > 0.1) ||
        (uv.x % 0.2 < 0.125 && uv.x % 0.2 > 0.1 && uv.y % 0.2 < 0.1)
    );

    let color = vec3(1.0, 0.05, 0.0) * val * bricks;

    return vec4<f32>(
        color,
        1.0
    );
}