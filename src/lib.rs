use std::ops::ControlFlow;

//WASM prelude
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    dpi::PhysicalSize, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .with_fullscreen(None)
    .build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut app = BasicApp::new(&window).await;

    event_loop
        .run(move |event, control_flow| match event {
            Event::Resumed => {
                log::debug!("Resumed");
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window().id() => 
            {
                //Process app input first
                if !app.input(event){
                    match event{
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            app.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            println!("Unhandled window rescaling!");
                        }
                        WindowEvent::RedrawRequested => {
                            app.update();
                            app.window().request_redraw();
                            match app.render(){
                                Ok(_) => {},
                                Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                                Err(e) => eprintln!("{:?}",e)
                            }

                        }
                        _ => {}
                    }
                }
            },
            _ => {}
        })
        .unwrap();
}

struct BasicApp<'a> {
    //Core api state
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a winit::window::Window,

    //Misc other state
    background_color: wgpu::Color
}

impl<'a> BasicApp<'a> {
    // Creating some of the wgpu types requires async code
    async fn new(window: &'a winit::window::Window) -> BasicApp<'a> {
        let size = window.inner_size();

        //Request backend api (GL on web, OS specific otherwise)
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                //Low power by default
                power_preference: wgpu::PowerPreference::default(),
                //Create an adapter compatible with the surface
                compatible_surface: Some(&surface),
                //Abort if wgpu tries to use software rendering
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        //Get surface capabilities
        let surface_caps = surface.get_capabilities(&adapter);
        
        //Check if the surface supports sRGB textures, if so get this
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        //Create a swapchain
        //present mode default is FIFO
        //alpha mode has to do with window transparency
        //Going to be using the swapchain texture view as a render attachment
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            //Swapchain size
            desired_maximum_frame_latency: 2,
            //Swizzle to access views in different format than underlying texture
            view_formats: vec![],
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            background_color: wgpu::Color{
                r: 0.541,
                g: 0.541,
                b: 0.506,
                a: 1.0
            }
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        //Recreate swapchain on resize
        if new_size.width > 0 && new_size.height > 0{
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device,&self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event{
            WindowEvent::CursorMoved { position ,..} => {
                println!("cursor position: {:?}",position);
                let x = position.x / self.size.width as f64;
                let y = position.y / self.size.height as f64;
                self.background_color = wgpu::Color{
                    r: x,
                    g: 0.0,
                    b: y,
                    a: 1.0 
                };
                true
            }
            _ => false
        }
    }

    fn update(&mut self) {
        
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        //Get texture view to render to from swapchain/surface
        let swapchain_texture = self.surface.get_current_texture()?;
        let swapchain_view = swapchain_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        //Prepare commands to send via a command buffer encoder

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("my cmd encoder")
        });

        //Set random state

        //Encode everything in the scope below
        {


            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("my render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &swapchain_view,
                    resolve_target: None,
                    ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(self.background_color),
                        store: wgpu::StoreOp::Store
                    }
                })],
                //No depth testing rn
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });

            //Render pass borrows encoder via begin_render_pass, so need to drop render pass
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        swapchain_texture.present();

        Ok(())
    }
}