struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
};

struct Matrix{
    inner_mat: mat4x4<f32>,
}

//Vertex shader uniforms
@group(1) @binding(0)
var<uniform> matrix: Matrix;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = matrix.inner_mat * vec4<f32>(in.position, 1.0);
    out.uv = in.uv;
    return out;
}

//Frag shader uniforms
//texture binding
@group(0) @binding(0)
var tex: texture_2d<f32>;
//sampler binding
@group(0) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex,tex_sampler,in.uv);
}