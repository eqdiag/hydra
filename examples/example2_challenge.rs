use hydra::{app::{App, EventHandler, Frame, Size}, context::Context};
use wgpu::{Backends, Color};
use winit::{event::ElementState, keyboard::KeyCode::*, window};


struct State{
  bg_color: wgpu::Color
}

fn init(_app: &App<State>,_context: &Context) -> State{
    State{
        bg_color: wgpu::Color::BLACK
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
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
            label: Some("my render pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    //For multisampling
                    resolve_target: None,
                    ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(state.bg_color),
                        store: wgpu::StoreOp::Store,
                    },
                })
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
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

fn mouse_move(state: &mut State,position: hydra::app::Position,size: Size,event_handler: EventHandler){
    let r = position.x / size.width as f64;
    let b = position.y / size.height as f64;
    state.bg_color = Color{r: r,g: 0.0,b: b,a: 1.0};
}



fn main(){
    App::new(init)
    .update(update)
    .render(render)
    .on_key(key_input)
    .on_mouse_move(mouse_move)
    .with_title("example2_renderpass".to_string())
    .run();
}