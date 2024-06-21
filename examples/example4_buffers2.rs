use hydra::{app::{App, EventHandler, Frame}, context::Context, pipeline::RenderPipelineBuilder, vertex::ColoredVertex};
use wgpu::{util::DeviceExt, Backends, IndexFormat, ShaderModule, ShaderSource, VertexBufferLayout};
use winit::{event::ElementState, keyboard::KeyCode::*, window};

const VERTICES: &[ColoredVertex] = &[
    ColoredVertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, 
    ColoredVertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, 
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];




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



fn update(state: &mut State){
    
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

fn key_input(state: &mut State,key: hydra::app::Key,key_state: ElementState,event_handler: EventHandler){
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
    .with_title("example4_buffers".to_string())
    .run();
}