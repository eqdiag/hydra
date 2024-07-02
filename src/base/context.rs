use colored::Colorize;
use wgpu::{Backends, PowerPreference};

pub struct Context<'a>{
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'a>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub config: wgpu::SurfaceConfiguration,

    init: bool,
}

impl<'a> Context<'a>{
   
    pub async fn new(window: &'a winit::window::Window) -> Context<'a>{
        //instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        //surface
        let surface = instance.create_surface(window).unwrap();

        //adapter
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await.unwrap();

        //device
        let (device,queue) = adapter.request_device(&wgpu::DeviceDescriptor{
            label: Some("my device"),
            required_features: wgpu::Features::default(),
            required_limits: wgpu::Limits::default(),
        },None).await.unwrap();

        //swapchain
        let surface_capabilities = surface.get_capabilities(&adapter);

        //prefer srgb, otherwise get first one
        let swapchain_format = surface_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        let size = window.inner_size();


        let config = wgpu::SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        //adapter features & limits

        println!("{}",&format!("Adapter Features: {:#?}",adapter.features())[..].green());
        println!("{}",&format!("Adapter Limits: {:#?}",adapter.features())[..].green());



        //surface present modes,formats, usages
        println!("{}",&format!("Surface [Present Modes]: {:#?}",surface_capabilities.present_modes)[..].green());
        println!("{}",&format!("Surface [Formats]: {:#?}",surface_capabilities.formats)[..].green());
        println!("{}",&format!("Surface [Usages]: {:#?}",surface_capabilities.usages)[..].green());



        Self{
            instance,
            surface,
            adapter,
            device,
            queue,
            size,
            config,
            init: true
        }
    }

    pub fn init(&self) -> bool{
        self.init
    }


}