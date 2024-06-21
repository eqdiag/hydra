use wgpu::VertexBufferLayout;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredVertex{
    pub position: [f32;3],
    pub color: [f32;3]
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexturedVertex{
    pub position: [f32;3],
    pub uv: [f32;2]
}

impl ColoredVertex{
    pub fn layout() -> VertexBufferLayout<'static>{
        VertexBufferLayout{
            array_stride: std::mem::size_of::<ColoredVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                //position attribute (location = 0)
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                //color attribute (location = 1)
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                }
            ],
        }
    }
}

impl TexturedVertex{
    pub fn layout() -> VertexBufferLayout<'static>{
        VertexBufferLayout{
            array_stride: std::mem::size_of::<TexturedVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                //position attribute (location = 0)
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                //texture attribute (location = 1)
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                }
            ],
        }
    }
}