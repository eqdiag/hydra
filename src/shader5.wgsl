//In
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput{
    @location(2) model_matrix_row0: vec4<f32>,
    @location(3) model_matrix_row1: vec4<f32>,
    @location(4) model_matrix_row2: vec4<f32>,
    @location(5) model_matrix_row3: vec4<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

//Out
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

//Uniforms
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

// Vertex shader

@vertex
fn vs_main(in: VertexInput,instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_row0,
        instance.model_matrix_row1,
        instance.model_matrix_row2,
        instance.model_matrix_row3
    );
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(in.position, 1.0);
    out.tex_coords = in.tex_coords;
    return out;
}

//Fragment shader
@group(0) @binding(0)
var diffuse_texture: texture_2d<f32>;
@group(0) @binding(1)
var diffuse_texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(diffuse_texture, diffuse_texture_sampler, in.tex_coords);
}