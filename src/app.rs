use colored::Colorize;
use wgpu::SurfaceTexture;
use winit::dpi::PhysicalPosition;
use winit::event::{DeviceEvent, DeviceId, ElementState, MouseButton};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::KeyEvent, event_loop::ActiveEventLoop, window::WindowAttributes};
use winit::keyboard::PhysicalKey::Code;

use crate::context::Context;

pub type Key = winit::keyboard::KeyCode;
pub type Frame = SurfaceTexture;
pub type Position = PhysicalPosition<f64>;
pub type Size = PhysicalSize<u32>;

pub struct EventHandler<'a>{
    control_flow: &'a ActiveEventLoop
}

impl<'a> EventHandler<'a>{
    pub fn exit(self){
        self.control_flow.exit();
    }
}

//App state
pub struct App<'a,T>{
    //surface: Option<wgpu::Surface<'window>>,
    context: Option<crate::context::Context<'a>>,
    window: Option<&'a winit::window::Window>,
    state: Option<T>,
    init_fn: fn(&App<T>,ctx: &Context) -> T,
    update_fn: Option<fn(state: &mut T,ctx: &Context)>,
    render_fn: Option<fn(state: &T,ctx: &Context,frame: Frame)>,

    //input functions
    on_key_fn: Option<fn(state: &mut T,key: Key,key_state: ElementState, event_handler: EventHandler)>,
    on_cursor_move_fn: Option<fn(state: &mut T,p: Position,size: Size,event_handler: EventHandler)>,
    on_mouse_move_fn: Option<fn(state: &mut T,delta: (f32,f32),event_handler: EventHandler)>,
    on_mouse_input_fn: Option<fn(state: &mut T,mouse_button: MouseButton,button_state: ElementState,event_handler: EventHandler)>,


    //misc customization
    title: String
}

impl<'window,T> App<'window,T>{

    pub fn new(init: fn(&App<T>,ctx: &Context) -> T) -> App<'window,T> {
        App::<T>{
            context: None,
            window: None,
            state:None,
            init_fn: init,
            update_fn: None,
            render_fn: None,
            on_key_fn: None,
            on_cursor_move_fn: None,
            on_mouse_move_fn: None,
            on_mouse_input_fn: None,
            title: "hydra app".to_string()}
    }

    pub fn context(&self) -> Option<&crate::context::Context>{
        self.context.as_ref()
    }

    pub fn window(&self) -> Option<&winit::window::Window>{
        self.window
    }


    pub fn update(mut self,f: fn(state: &mut T,ctx: &Context)) -> Self{
        self.update_fn = Some(f);
        self
    }

    pub fn render(mut self,f: fn(state: &T,&Context,frame: Frame)) -> Self{
        self.render_fn = Some(f);
        self
    }

    pub fn on_key(mut self,f: fn(state: &mut T,key: Key,key_state: ElementState,event_handler: EventHandler)) -> Self{
        self.on_key_fn = Some(f);
        self
    }

    pub fn on_cursor_move(mut self,f: fn(state: &mut T,p: Position,size: Size,event_handler: EventHandler)) -> Self{
        self.on_cursor_move_fn = Some(f);
        self
    }

    pub fn on_mouse_move(mut self,f: fn(state: &mut T,delta: (f32,f32),event_handler: EventHandler)) -> Self{
        self.on_mouse_move_fn = Some(f);
        self
    }

    pub fn on_mouse_input(mut self,f: fn(state: &mut T,mouse_button: MouseButton,button_state: ElementState,event_handler: EventHandler)) -> Self{
        self.on_mouse_input_fn = Some(f);
        self
    }
    

    pub fn with_title(mut self,title: String) -> Self{
        self.title = title;
        self
    }

    async fn inner_run<'a>(mut self,window:&'a winit::window::Window,event_loop: winit::event_loop::EventLoop<()>){
        
    
        //Create a context here
        let context = crate::context::Context::new(window).await;
        self.context = Some(context);

        self.window = Some(&window);

        
        event_loop.run_app(&mut self).unwrap();

    }

    pub fn run(self){
        
        let event_loop = winit::event_loop::EventLoop::new().unwrap();

        let window_attributes = WindowAttributes::default()
            .with_title(&self.title);


        let window = event_loop.create_window(window_attributes).unwrap();

        pollster::block_on(self.inner_run(&window,event_loop));
    }
}

//Input handling
impl<'window,T> ApplicationHandler for App<'window,T>{
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        match self.state{
            Some(_) => {},
            None => {
                self.state = Some((self.init_fn)(&self,self.context.as_ref().unwrap()));
            },
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event{
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::KeyboardInput {event: KeyEvent{ physical_key: Code(key),state,..},..} => {
                if let Some(f) = self.on_key_fn{
                    if let Some(app_state) = self.state.as_mut(){
                        let event_handler = EventHandler{control_flow: event_loop};
                        f(app_state,key,state,event_handler);
                    }
                    
                }
            },
            winit::event::WindowEvent::CursorMoved { position ,..} => {
                if let Some(f) = self.on_cursor_move_fn{
                    if let Some(state) = self.state.as_mut(){
                        let event_handler: EventHandler = EventHandler{control_flow: event_loop};
                        if let Some(win) = self.window{
                            let size =  win.inner_size();
                            f(state,position,size,event_handler);
                        }
                        
                    }
                    
                }
            }
            winit::event::WindowEvent::MouseInput { state, button,.. } => {
                if let Some(f) = self.on_mouse_input_fn{
                    if let Some(app_state) = self.state.as_mut(){
                        let event_handler: EventHandler = EventHandler{control_flow: event_loop};
                        f(app_state,button,state,event_handler);
                    }
                    
                }
            }
            winit::event::WindowEvent::Resized(PhysicalSize{width,height}) => {
                println!("{}",&format!("Resized : ({width},{height})")[..].green());
                if let Some(win) = &self.window{
                    win.request_redraw();
                }
            }
            winit::event::WindowEvent::RedrawRequested => {
                if let Some(win) = &self.window{
                    win.request_redraw();
                    
                }

                //call update
                if let Some(f) = self.update_fn{
                    if let Some(state) = self.state.as_mut(){
                        f(state,self.context.as_ref().unwrap());
                    }
                }

                //call render
                if let Some(f) = self.render_fn{
                    if let Some(state) = self.state.as_ref(){
                        let surface_texture = self.context.as_ref().unwrap().surface.get_current_texture().unwrap();
                        f(state,self.context.as_ref().unwrap(),surface_texture);
                    }
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event{
            DeviceEvent::MouseMotion {delta} => {
                if let Some(f) = self.on_mouse_move_fn{
                    if let Some(state) = self.state.as_mut(){
                        let event_handler = EventHandler{control_flow: event_loop};
                        f(state,(delta.0 as f32,delta.1 as f32),event_handler);
                    }
                    
                }
            },
            _ => {}
        }
    }
}



