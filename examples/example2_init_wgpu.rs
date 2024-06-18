use wgpu::core::device::queue;
use wgpu::{Backends, PowerPreference};
use winit::dpi::PhysicalSize;
use winit::event::DeviceEvent;
use winit::{application::ApplicationHandler, event::KeyEvent, event_loop::ActiveEventLoop, keyboard::KeyCode, window::WindowAttributes};
use winit::keyboard::PhysicalKey::Code;
use winit::event::DeviceId;


struct App<'a>{
    window: &'a winit::window::Window,
    ctx: hydra::util::context::Context<'a>,
    //bg color
    background_color: wgpu::Color
}

impl<'a> App<'a>{
    pub fn new(window: &'a winit::window::Window, ctx: hydra::util::context::Context<'a>) -> App<'a>{
        App{window,ctx,background_color: wgpu::Color::BLACK}
    }

    pub fn set_background_color(&mut self,color: wgpu::Color){
        self.background_color = color;
    }
}

impl<'a> ApplicationHandler for App<'a>{
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        //
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event{
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::KeyboardInput {event: KeyEvent{ physical_key: Code(KeyCode::Escape),..},..} => event_loop.exit(),
            winit::event::WindowEvent::CursorMoved {position,.. } => {
                //note: don't use this for camera, use device events instead
                println!("pos: {:?}",position);
                let r = position.x/ self.ctx.size.width as f64;
                let b = position.y/ self.ctx.size.height as f64;
                //println!(wgpu::Color{r,g: 0.0,b,a: 1.0});
                self.set_background_color(wgpu::Color{r,g: 0.0,b,a: 1.0});
            }

            winit::event::WindowEvent::Resized(PhysicalSize{width,height}) => {
                println!("Resized ({},{})",width,height);
                //on mac must manually call redraw after resize
                self.window.request_redraw();
            }
            winit::event::WindowEvent::RedrawRequested => {

                //Start next frame
                self.window.request_redraw();

                let frame = self.ctx.surface.get_current_texture().expect("Couldn't get texture!");
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
                    label: Some("my encoder"),
                });

                {
                    let mut _rpass =
                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(self.background_color),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                }

                self.ctx.queue.submit(Some(encoder.finish()));
                frame.present();

            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event{
            DeviceEvent::MouseMotion { delta } => {
                //println!("del: ({},{})",delta.0,delta.1);
            },
            _ => {}
        }
    }

}



async fn run(){
    //Event loop + window
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let window_attributes = WindowAttributes::default()
        .with_title("hydra (demo)");

    let window = event_loop.create_window(window_attributes).unwrap();

    let ctx = hydra::util::context::Context::new(&window).await;


    //Create custom app
    let mut app = App::new(&window, ctx);
    event_loop.run_app(&mut app).unwrap();
}

fn main(){
    pollster::block_on(run());    
}

/*
    todo:
    list adapter
        - features
        - limits
    list surface
        - present modes
        - formats
        - usages

 */