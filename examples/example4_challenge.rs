use hydra::{app::{App, EventHandler, Frame}, context::Context, pipeline::RenderPipelineBuilder};
use wgpu::{util::DeviceExt, Backends, IndexFormat, ShaderModule, ShaderSource, VertexBufferLayout};
use winit::{event::ElementState, keyboard::KeyCode::*, window};

const VERTICES: &[ColoredVertex] = &[
    ColoredVertex { position: [-0.5,-0.5, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [-0.25,-0.5, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [-0.25,0.5, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [-0.5,0.5, 0.0], color: [0.5, 0.0, 0.5] }, 

    ColoredVertex { position: [0.25,-0.5, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [0.75,-0.5, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [0.75,0.5, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [0.25,0.5, 0.0], color: [0.5, 0.0, 0.5] }, 
];

const INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3,
    4, 5, 7,
    5, 6, 7
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ColoredVertex{
    position: [f32;3],
    color: [f32;3]
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


struct State{
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer
}

fn init(_app: &App<State>,ctx: &Context) -> State{


    //create buffers
    let vertex_buffer = ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("my vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    let index_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("my index buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    //pipeline layout
    let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("my pipeline layout"),
        //for binding buffers,textures
        bind_group_layouts: &[],
        //for pushing uniform data via commands (small data)
        push_constant_ranges: &[],
    });


    let color_target = wgpu::ColorTargetState{
        format: ctx.config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    };

    let pipeline = RenderPipelineBuilder::new(ctx)
        .with_shaders(ShaderSource::Wgsl(include_str!("../assets/example4_shader.wgsl").into()), "vs_main", "fs_main")
        .with_culling(None, wgpu::FrontFace::Ccw)
        .with_layout(pipeline_layout)
        .add_vertex_buffer_layout(ColoredVertex::layout())
        .add_color_target_state(color_target)
        .build();
        

    State{
        pipeline,
        vertex_buffer,
        index_buffer
    }
}



fn update(state: &mut State,ctx: &Context){
    
}

fn render(state: &State,ctx: &Context,frame: Frame){
    
    //texture view to render to
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
        label: Some("my cmd encoder"),
    });

    {
        //render pass borrows the encoder, so need to drop the pass after we're done with it for encoder.finish() call
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
            label: Some("my render pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    //For multisampling
                    resolve_target: None,
                    ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: wgpu::StoreOp::Store,
                    },
                })
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        //bind resources (buffers,images)
        render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(state.index_buffer.slice(..),IndexFormat::Uint16);

        //bind pipeline
        render_pass.set_pipeline(&state.pipeline);
        //make render calls
        render_pass.draw_indexed(0..(INDICES.len() as u32),0, 0..1);
    }

    ctx.queue.submit(std::iter::once(encoder.finish()));
    frame.present();

}

fn key_input(state: &mut State,key: hydra::app::Key,key_state: ElementState,event_handler: &EventHandler){
    println!("key: {:#?}",key);
    match key{
        Escape => event_handler.exit(),
        _ => {}
    }
}



fn main(){
    App::new(init)
    .update(update)
    .render(render)
    .on_key(key_input)
    .with_title("example4_challenge".to_string())
    .run();
}