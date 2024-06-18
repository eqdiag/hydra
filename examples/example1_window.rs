use wgpu::Backends;
use winit::dpi::PhysicalSize;
use winit::{application::ApplicationHandler, event::KeyEvent, event_loop::ActiveEventLoop, keyboard::KeyCode, window::WindowAttributes};
use winit::keyboard::PhysicalKey::Code;

struct App{

}

impl ApplicationHandler for App{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("resumed");
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
            winit::event::WindowEvent::Resized(PhysicalSize{width,height}) => {
                println!("Resized ({},{})",width,height);
            }
            _ => {}
        }
    }
}

fn main(){

    //Event loop + window
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let window_attributes = WindowAttributes::default()
        .with_title("hydra (demo)");

    let window = event_loop.create_window(window_attributes).unwrap();

    //Create custom app
    let mut app = App{};
    event_loop.run_app(&mut app).unwrap();
}