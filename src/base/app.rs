use colored::Colorize;
use wgpu::SurfaceTexture;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent,ElementState, Event, KeyEvent, MouseButton};
use winit::event_loop::EventLoopWindowTarget;
use winit::keyboard::PhysicalKey::Code;
use winit::window::WindowBuilder;
use crate::base::context::Context;
use crate::core::ui::{self, Ui};

pub type Key = winit::keyboard::KeyCode;
pub type Frame = SurfaceTexture;
pub type Position = PhysicalPosition<f64>;
pub type Size = PhysicalSize<u32>;
pub type EventHandler = EventLoopWindowTarget<()>;


//App state
pub struct App<'a,T>{
    context: Option<crate::base::context::Context<'a>>,
    window: Option<&'a winit::window::Window>,
    ui: Option<Ui>,
    state: Option<T>,
    init_fn: fn(&App<T>,ctx: &Context) -> T,
    update_fn: Option<fn(state: &mut T,ctx: &Context)>,
    render_fn: Option<fn(state: &T,ctx: &Context,frame: Frame)>,
    render_with_ui_fn: Option<fn(state: &T,ui: &mut ui::Ui, ctx: &Context,frame: Frame)>,
    on_window_resize: Option<fn(state: &mut T,ctx: &Context,width: u32,height: u32)>,


    //input functions
    on_key_fn: Option<fn(state: &mut T,key: Key,key_state: ElementState,control: &EventLoopWindowTarget<()>)>,
    on_cursor_move_fn: Option<fn(state: &mut T,p: Position,size: Size,control: &EventLoopWindowTarget<()>)>,
    on_mouse_move_fn: Option<fn(state: &mut T,delta: (f32,f32),control: &EventLoopWindowTarget<()>)>,
    on_mouse_input_fn: Option<fn(state: &mut T,mouse_button: MouseButton,button_state: ElementState,control: &EventLoopWindowTarget<()>)>,


    //misc customization
    title: String,
    
}

impl<'window,T> App<'window,T>{

    pub fn new(init: fn(&App<T>,ctx: &Context) -> T) -> App<'window,T> {
        App::<T>{
            context: None,
            window: None,
            ui: None,
            state:None,
            init_fn: init,
            update_fn: None,
            render_fn: None,
            render_with_ui_fn: None,
            on_window_resize: None,
            on_key_fn: None,
            on_cursor_move_fn: None,
            on_mouse_move_fn: None,
            on_mouse_input_fn: None,
            title: "hydra app".to_string()
        }
    }

    pub fn context(&self) -> Option<&crate::base::context::Context>{
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
        if let Some(_) = self.render_with_ui_fn{
            panic!("Can't have both a render and render_with_ui function!");
        }
        self
    }

    pub fn render_with_ui(mut self,f: fn(state: &T,ui: &mut ui::Ui,&Context,frame: Frame)) -> Self{
        self.render_with_ui_fn = Some(f);
        if let Some(_) = self.render_fn{
            panic!("Can't have both a render and render_with_ui function!");
        }

        self
    }

    pub fn on_window_resize(mut self,f: fn(state: &mut T,ctx: &Context,width: u32,height: u32)) -> Self{
        self.on_window_resize = Some(f);
        self
    }


    pub fn on_key(mut self,f: fn(state: &mut T,key: Key,key_state: ElementState,control: &EventLoopWindowTarget<()>)) -> Self{
        self.on_key_fn = Some(f);
        self
    }

    pub fn on_cursor_move(mut self,f: fn(state: &mut T,p: Position,size: Size,control: &EventLoopWindowTarget<()>)) -> Self{
        self.on_cursor_move_fn = Some(f);
        self
    }

    pub fn on_mouse_move(mut self,f: fn(state: &mut T,delta: (f32,f32),control: &EventLoopWindowTarget<()>)) -> Self{
        self.on_mouse_move_fn = Some(f);
        self
    }

    pub fn on_mouse_input(mut self,f: fn(state: &mut T,mouse_button: MouseButton,button_state: ElementState,control: &EventLoopWindowTarget<()>)) -> Self{
        self.on_mouse_input_fn = Some(f);
        self
    }
    

    pub fn with_title(mut self,title: String) -> Self{
        self.title = title;
        self
    }

    async fn inner_run<'a>(mut self,window:&'a winit::window::Window,event_loop: winit::event_loop::EventLoop<()>){
        


        //Create a context here
        let context = crate::base::context::Context::new(window).await;
        self.context = Some(context);

        self.window = Some(&window);
        
        event_loop.run(move |event,control_flow|{

            let mut ui_handle = false;
            if let Some(ui) = self.ui.as_mut(){
                ui.platform.handle_event(&event);
                if ui.platform.captures_event(&event) {
                    ui_handle = true;
                }
            }
            
            if !ui_handle{
                match event{
                    Event::WindowEvent { window_id, event } => {
                        match event{
                            winit::event::WindowEvent::CloseRequested => {control_flow.exit()},
                            winit::event::WindowEvent::KeyboardInput {event: KeyEvent{ physical_key: Code(key),state,..},..} => {
                                if let Some(f) = self.on_key_fn{
                                    if let Some(app_state) = self.state.as_mut(){
                                        f(app_state,key,state,control_flow);
                                    }
                                    
                                }
                            },
                            winit::event::WindowEvent::CursorMoved { position ,..} => {
                                if let Some(f) = self.on_cursor_move_fn{
                                    if let Some(state) = self.state.as_mut(){
                                        if let Some(win) = self.window{
                                            let size =  win.inner_size();
                                            f(state,position,size,control_flow);
                                        }
                                        
                                    }
                                    
                                }
                            }
                            winit::event::WindowEvent::MouseInput { state, button,.. } => {
                                if let Some(f) = self.on_mouse_input_fn{
                                    if let Some(app_state) = self.state.as_mut(){
                                        f(app_state,button,state,control_flow);
                                    }
                                    
                                }
                            }
                            winit::event::WindowEvent::Resized(size @ PhysicalSize{width,height}) => {
                                println!("{}",&format!("Resized : ({width},{height})")[..].green());
                                if width > 0 && height > 0{
                                    let ctx = self.context.as_mut().unwrap();
                                    ctx.size = size;
                                    ctx.config.width = width;
                                    ctx.config.height = height;
                                    ctx.surface.configure(&ctx.device,&ctx.config);
                                }
                                if let Some(win) = &self.window{
                                    win.request_redraw();
                                }
                                //call user provided resize function
                                if let Some(f) = self.on_window_resize{
                                    if let Some(state) = self.state.as_mut(){
                                        f(state,self.context.as_ref().unwrap(), width,height);
                                    }
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
                
                                //call render with ui (only one will actually be called)
                                if let Some(f) = self.render_with_ui_fn{
                                    if let Some(state) = self.state.as_mut(){
                                        let surface_texture = self.context.as_ref().unwrap().surface.get_current_texture().unwrap();
                                        f(state,self.ui.as_mut().unwrap(), self.context.as_ref().unwrap(),surface_texture);
                                    }
                                }
                            },
                            winit::event::WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                                //TODO: inform egui
                            }
                            _ => {}
                        }
                    },
                    Event::DeviceEvent { device_id, event } => {
                        match event{
                            DeviceEvent::MouseMotion {delta} => {
                                if let Some(f) = self.on_mouse_move_fn{
                                    if let Some(state) = self.state.as_mut(){
                                        f(state,(delta.0 as f32,delta.1 as f32),control_flow);
                                    }
                                    
                                }
                            },
                            _ => {}
                        }
                    },
                    Event::Resumed => {
                        //create ui here, only create ui if the user specifies a ui render function
                        if let Some(_) = self.render_with_ui_fn{
                            self.ui = Some(ui::Ui::new(self.context.as_ref().unwrap(),self.window.unwrap().scale_factor()));
                        }

                        //init app state
                        self.state = Some((self.init_fn)(&self,self.context.as_ref().unwrap()));
                    },
                    _ => {}
                }
            }
        }).unwrap();
    }

    pub fn run(self){
        
        let event_loop = winit::event_loop::EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_title(&self.title)
            .build(&event_loop).unwrap();
       

        pollster::block_on(self.inner_run(&window,event_loop));
    }
}




