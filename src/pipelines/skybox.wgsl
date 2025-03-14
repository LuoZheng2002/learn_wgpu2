// Vertex Shader
struct CameraUniform {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;
struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
};
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {

    let view = camera.view;
    let view_without_translation = mat4x4<f32>(
        view[0][0], view[0][1], view[0][2], 0.0,
        view[1][0], view[1][1], view[1][2], 0.0,
        view[2][0], view[2][1], view[2][2], 0.0,
        0.0, 0.0, 0.0, 1.0
    );


    var out: VertexOutput;
    out.tex_coords = input.position;
    let transformed_pos = camera.projection * view_without_translation * vec4<f32>(input.position, 1.0);
    // set z to 1
    out.position = transformed_pos.xyww;
    return out;
}

@group(0) @binding(0)
var cubemap_texture: texture_cube<f32>;
@group(0) @binding(1)
var cubemap_sampler: sampler;
// Fragment Shader (for cubemap)
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(cubemap_texture, cubemap_sampler, in.tex_coords);
}