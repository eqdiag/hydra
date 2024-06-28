struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
}

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) clip_position: vec4<f32>
};



@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.uv = in.uv;
    return out;
}

//Frag shader uniforms
//depth texture binding
@group(0) @binding(0)
var tex: texture_2d<f32>;

@group(0) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //NOTE: depth is stored in the red channel
    var depth: f32 = textureSample(tex,tex_sampler,in.uv).r;
    return vec4<f32>(vec3<f32>(depth),1.0);
    //return vec4<f32>(in.uv,0.0,0.0);
}