use wgpu::RequestAdapterOptions;
use winit::{dpi::PhysicalSize, event::{ElementState, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


struct App<'a>{
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    //Swapchain size
    size: winit::dpi::PhysicalSize<u32>,
    //Beware: surface contains window references (surface must be destroyed first)
    window: &'a winit::window::Window,

    //Actual app state
    bg_color: wgpu::Color
}

impl<'a> App<'a> {
    async fn new(window: &'a winit::window::Window) -> App<'a>{
        
        let size = window.inner_size();

        //Instance
        //Validation and shader debugging turned enabled by default
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        //Surface
        let surface = unsafe{ instance.create_surface(window)}.unwrap();

        //Adapter
        //Adapter = hardware device + api, ex: linux gpu = 2 adapters (one opengl, one vulkan)
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            //Doesn't choose between low/high power mode
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        println!("ADAPTER FEATURES: {:#?}",adapter.features());

        //Device and Queue
        let (device,queue) = adapter.request_device(
            &wgpu::DeviceDescriptor{
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
        None
        ).await.unwrap();

        //Swapchain creation (though wgpu hides this, its what we're doing though)
        let surface_capabilites = surface.get_capabilities(&adapter);
        //Check for sRGB swapchain format, otherwise choose first supported as default
        let surface_format = surface_capabilites.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilites.formats[0]);

        //Swapchain config
        let config = wgpu::SurfaceConfiguration{
            //Use of swapchain images
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            //Swapchain texture format and dimensions
            format: surface_format,
            width: size.width,
            height: size.height,
            //Swapchain present modes (default = FIFO)
            present_mode: surface_capabilites.present_modes[0],
            //For window transparency
            alpha_mode: surface_capabilites.alpha_modes[0],
            //Allows us to swizzle format of swapchain images
            view_formats: vec![],
            //Num swapchain images
            desired_maximum_frame_latency: 2
        };

        Self{
            window,
            surface,
            device,
            queue,
            config,
            size,
            bg_color: wgpu::Color{r: 1.0,g: 0.0,b: 0.0,a: 1.0}
        }

    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
        println!("new size: {:?}",new_size);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event{
            WindowEvent::CursorMoved { position,..} => {
                let r = position.x / self.size.width as f64;
                let b = position.y / self.size.height as f64;
                self.bg_color = wgpu::Color{r,b,..Default::default()};
                println!("pos: {:?}",position);
            }
            _ => {}
        }
        false
    }

    fn update(&mut self) {
        
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("my cmd encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("my render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.bg_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}



//Entry point for wasm
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run(){

    cfg_if::cfg_if!{
        //For js/wasm build
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            //Only works on native
            env_logger::init();
        }
    }

    //Keeps track of OS events
    let event_loop = EventLoop::new().unwrap();

    

    //OS independent window
    let window = WindowBuilder::new()
        .with_title("hydra demo")
        .build(&event_loop)
        .unwrap();

    //Give wasm access to canvas
    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        //Set size of canvas here!!!
        //window.window.request_inner_size(PhysicalSize::new(450, 400));
        
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

    let mut app = App::new(&window).await;
    let mut surface_configured = false;

    event_loop
    .run(move |event, control_flow| {
        match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window().id() => {
                if !app.input(event) {
                    match event {


                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),


                        WindowEvent::Resized(physical_size) => {
                            log::info!("physical_size: {physical_size:?}");
                            surface_configured = true;
                            app.resize(*physical_size);
                        }


                        WindowEvent::RedrawRequested => {
                            // This tells winit that we want another frame after this one
                            app.window().request_redraw();

                            if !surface_configured {
                                return;
                            }

                            app.update();
                            match app.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(
                                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                ) => app.resize(app.size),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    log::error!("OutOfMemory");
                                    control_flow.exit();
                                }

                                // This happens when the a frame takes too long to present
                                Err(wgpu::SurfaceError::Timeout) => {
                                    log::warn!("Surface timeout")
                                }
                            }
                        }

                        WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => {
                            println!("Unhandled scale size");
                        }

                        _ => {}
                    }
                }
            }

            _ => {}
        }
    })
    .unwrap();

}