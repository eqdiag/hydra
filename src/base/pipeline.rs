use std::num::NonZeroU32;

use wgpu::{core::id::markers::PipelineLayout, hal::DepthStencilAttachment, Face, FrontFace, ShaderModule, ShaderSource, VertexBufferLayout};

use crate::base::context;

pub struct RenderPipelineBuilder<'a>{

    //context
    context: &'a crate::base::context::Context<'a>,

    //pipeline layout
    layout: Option<wgpu::PipelineLayout>,

    //vertex state structures
    shader_module: Option<wgpu::ShaderModule>,
    vertex_entry: Option<&'a str>,
    fragment_entry: Option<&'a str>,
    vertex_buffer_layouts: Vec<VertexBufferLayout<'a>>,

    //primitive state
    primitive: wgpu::PrimitiveState,

    //depth stencil state
    depth_stencil: Option<wgpu::DepthStencilState>,

    //multisample state
    multisample: wgpu::MultisampleState,

    //fragment state structures
    color_targets: Vec<Option<wgpu::ColorTargetState>>,

    //multiview state
    multiview: Option<NonZeroU32>
}

impl<'a> RenderPipelineBuilder<'a>{

    pub fn new(context: &'a crate::base::context::Context<'a>) -> Self{
        RenderPipelineBuilder{
            context: context,
            layout: None,
            shader_module: None,
            vertex_entry: None,
            fragment_entry: None,
            vertex_buffer_layouts: vec![],
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(), 
            color_targets: vec![],
            multiview: None
        }
    }


    pub fn with_layout(mut self,layout: wgpu::PipelineLayout) -> Self{
        self.layout = Some(layout);
        self
    }

    pub fn with_shaders(mut self,shader_source: ShaderSource,vertex_entry: &'a str,fragment_entry: &'a str) -> Self{
        self.vertex_entry = Some(&vertex_entry);
        self.fragment_entry = Some(&fragment_entry);

        self.shader_module = Some(self.context.device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("my shader module"),
            source: shader_source,
        }));

        self
    }

    pub fn add_vertex_buffer_layout(mut self,layout: VertexBufferLayout<'a>) -> Self{
        self.vertex_buffer_layouts.push(layout);
        self
    }

    pub fn with_culling(mut self,cull_mode: Option<Face>,front_face: FrontFace) -> Self{
        self.primitive.cull_mode = cull_mode;
        self.primitive.front_face = front_face;
        self
    }

    pub fn with_depth_stencil_state(mut self,state: wgpu::DepthStencilState) -> Self{
        self.depth_stencil = Some(state);
        self
    }

    pub fn add_color_target_state(mut self,target: wgpu::ColorTargetState) -> Self{
        self.color_targets.push(Some(target));
        self
    }

    pub fn build(self) -> wgpu::RenderPipeline{

        let vertex_state = wgpu::VertexState{
            module: self.shader_module.as_ref().expect("pipeline creation: no shader module"),
            entry_point: self.vertex_entry.as_ref().expect("pipeline creation: no shader module entry point"),
            compilation_options: Default::default(),
            buffers: &self.vertex_buffer_layouts[..],
        };

        let fragment_state = wgpu::FragmentState{
            module: self.shader_module.as_ref().expect("pipeline creation: no shader module"),
            entry_point: self.fragment_entry.as_ref().expect("pipeline creation: no shader module entry point"),
            compilation_options: Default::default(),
            targets: &self.color_targets[..],
        };

        let descriptor = wgpu::RenderPipelineDescriptor{
            label: Some("my render pipeline"),
            layout: self.layout.as_ref(),
            vertex: vertex_state,
            primitive: self.primitive,
            depth_stencil: self.depth_stencil,
            multisample: self.multisample,
            fragment: Some(fragment_state),
            multiview: self.multiview,
        };

        self.context.device.create_render_pipeline(&descriptor)
    }
}