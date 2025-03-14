// Vertex Shader
struct ViewUniform {
    view: mat4x4<f32>,
}
@group(1) @binding(0) // 1.
var<uniform> view: ViewUniform;
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.position = view.view * vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var cubemap_texture: texture_cube<f32>;
@group(0) @binding(1)
var sampler: sampler;
// Fragment Shader (for cubemap)
@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(cubemap_texture, sampler, tex_coords);
}