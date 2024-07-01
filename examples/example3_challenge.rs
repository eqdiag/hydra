use hydra::{app::{App, EventHandler, Frame}, context::Context, pipeline::RenderPipelineBuilder};
use wgpu::{Backends, ShaderModule, ShaderSource};
use winit::{event::ElementState, keyboard::KeyCode::*, window};


struct State{
    pipeline0: wgpu::RenderPipeline,
    pipeline1: wgpu::RenderPipeline,
    pipeline_num: u32
}

fn init(_app: &App<State>,ctx: &Context) -> State{



    //pipeline layout
    let pipeline_layout0 = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("my pipeline layout"),
        //for binding buffers,textures
        bind_group_layouts: &[],
        //for pushing uniform data via commands (small data)
        push_constant_ranges: &[],
    });


    let color_target0 = wgpu::ColorTargetState{
        format: ctx.config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    };

    let pipeline0 = RenderPipelineBuilder::new(ctx)
        .with_shaders(ShaderSource::Wgsl(include_str!("../assets/example3_shader.wgsl").into()), "vs_main", "fs_main")
        .with_culling(None, wgpu::FrontFace::Ccw)
        .with_layout(pipeline_layout0)
        .add_color_target_state(color_target0)
        .build();

    //pipeline layout
    let pipeline_layout1 = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("my pipeline layout"),
        //for binding buffers,textures
        bind_group_layouts: &[],
        //for pushing uniform data via commands (small data)
        push_constant_ranges: &[],
    });


    let color_target1 = wgpu::ColorTargetState{
        format: ctx.config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    };

    let pipeline1 = RenderPipelineBuilder::new(ctx)
        .with_shaders(ShaderSource::Wgsl(include_str!("../assets/example3_shader_challenge.wgsl").into()), "vs_main", "fs_main")
        .with_culling(None, wgpu::FrontFace::Ccw)
        .with_layout(pipeline_layout1)
        .add_color_target_state(color_target1)
        .build();
        

    State{
        pipeline0,
        pipeline1,
        pipeline_num: 0
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

        //bind pipeline
        if state.pipeline_num == 0{
            render_pass.set_pipeline(&state.pipeline0);
        }else{
            render_pass.set_pipeline(&state.pipeline1);
        }
        //make render calls
        render_pass.draw(0..3, 0..1);
    }

    ctx.queue.submit(std::iter::once(encoder.finish()));
    frame.present();

}

fn key_input(state: &mut State,key: hydra::app::Key,key_state: ElementState, event_handler: &EventHandler){
    println!("key: {:#?}",key);
    match key{
        Escape => event_handler.exit(),
        Space => {
            match key_state{
                ElementState::Pressed => {
                    state.pipeline_num = (state.pipeline_num + 1) % 2;
                },
                ElementState::Released => {},
            }
        }
        _ => {}
    }
}



fn main(){
    App::new(init)
    .update(update)
    .render(render)
    .on_key(key_input)
    .with_title("example3_challenge".to_string())
    .run();
}