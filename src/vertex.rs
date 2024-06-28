use wgpu::VertexBufferLayout;

pub trait VertexLayout{
    fn new() -> Self;
    fn layout() -> VertexBufferLayout<'static>;
    fn add_position(&mut self,position: [f32;3]);
    fn try_add_normal(&mut self,normal: [f32;3]) -> Result<(),&str>;

    fn try_add_color(&mut self,normal: [f32;3]) -> Result<(),&str>{
        self.try_add_normal(normal)
    }

    fn try_add_uv(&mut self,uv: [f32;2]) -> Result<(),&str>;
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BasicVertex{
    pub position: [f32;3],
}

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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BasicInstanceData{
    data: [[f32;4];4]
}

impl VertexLayout for BasicVertex{
    fn layout() -> VertexBufferLayout<'static>{
        VertexBufferLayout{
            array_stride: std::mem::size_of::<BasicVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                //position attribute (location = 0)
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
            ],
        }
    }
    
    fn add_position(&mut self,position: [f32;3]){
        self.position = position;
    }
    
    fn try_add_normal(&mut self,normal: [f32;3]) -> Result<(),&str> {
        Err("BasicVertex doesn't support normals!")
    }
    
    fn try_add_uv(&mut self,position: [f32;2]) -> Result<(),&str> {
        Err("BasicVertex doesn't support uvs!")
    }
    
    fn new() -> Self {
        Self{
            position: Default::default()
        }
    }
    
}

impl VertexLayout for ColoredVertex{
    fn layout() -> VertexBufferLayout<'static>{
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
    
    fn add_position(&mut self,position: [f32;3]){
        self.position = position;
    }
    
    fn try_add_normal(&mut self,normal: [f32;3]) -> Result<(),&str> {
        self.color = normal;
        Ok(())
    }
    
    fn try_add_uv(&mut self,_uv: [f32;2]) -> Result<(),&str> {
        Err("BasicVertex doesn't support uvs!")
    }

    fn new() -> Self {
        Self{
            position: Default::default(),
            color: Default::default()
        }
    }
}

impl VertexLayout for TexturedVertex{
    fn layout() -> VertexBufferLayout<'static>{
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
    
    fn add_position(&mut self,position: [f32;3]){
        self.position = position;
    }
    
    fn try_add_normal(&mut self,_normal: [f32;3]) -> Result<(),&str> {
        Err("BasicVertex doesn't support normals!")
    }
    
    fn try_add_uv(&mut self,uv: [f32;2]) -> Result<(),&str> {
        self.uv = uv;
        Ok(())
    }

    fn new() -> Self {
        Self{
            position: Default::default(),
            uv: Default::default()
        }
    }
}

impl BasicInstanceData{
    pub fn layout() -> VertexBufferLayout<'static>{
        VertexBufferLayout{
            array_stride: std::mem::size_of::<BasicInstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                }
            ],
        }
    }
}