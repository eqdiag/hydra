use cgmath::{InnerSpace, Rotation3};
use image::GenericImageView;
use instance::{Instance, InstanceRaw};
use wgpu::{util::BufferInitDescriptor, BlendState, ColorTargetState, FragmentState, MultisampleState, RenderPassDepthStencilAttachment, RequestAdapterOptions};
use winit::{dpi::PhysicalSize, event::{ElementState, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder};
use wgpu::util::DeviceExt;


#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub mod camera;
pub mod instance;


#[repr(C)]
#[derive(Clone, Copy,Debug,bytemuck::Pod,bytemuck::Zeroable)]
struct BasicVertex{
     position: [f32;3],
     color: [f32;3]
}


impl BasicVertex{

    fn layout() -> wgpu::VertexBufferLayout<'static>{
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<BasicVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[                
                //position
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                //color
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                }
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy,Debug,bytemuck::Pod,bytemuck::Zeroable)]
struct TextureVertex{
     position: [f32;3],
     tex_coords: [f32;2]
}

impl TextureVertex{

    fn layout() -> wgpu::VertexBufferLayout<'static>{
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[                
                //position
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                //color
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                }
            ],
        }
    }
}


const VERTICES: &[BasicVertex] = &[
    BasicVertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    BasicVertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    BasicVertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

const VERTICES2: &[BasicVertex] = &[
    BasicVertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    BasicVertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    BasicVertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    BasicVertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    BasicVertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

const VERTICES3: &[TextureVertex] = &[
    TextureVertex{ position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    TextureVertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    TextureVertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    TextureVertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    TextureVertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

const INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DX: cgmath::Vector3<f32> = cgmath::Vector3::new(INSTANCES_PER_ROW as f32 * 0.5, 0.0, INSTANCES_PER_ROW as f32 * 0.5);

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
    bg_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,

    //Resources
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,

    index_buffer: wgpu::Buffer,
    num_indices: u32,

    //Material stuff
    diffuse_bind_group: wgpu::BindGroup,

    //Camera
    camera_controller: camera::CameraController,
    camera: camera::Camera,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    //Instancing
    instances: Vec<instance::Instance>,
    instance_buffer: wgpu::Buffer,

    //Depth texture stuff
    depth_texture: wgpu::Texture,
    depth_texture_view: wgpu::TextureView,

    //Misc state info
    time: f32,
}



impl<'a> App<'a> {
    async fn new(window: &'a winit::window::Window) -> App<'a>{
        
        let size = window.inner_size();

        //Backend logic based on web/os
        let backend = if cfg!(target_arch = "wasm32"){
            wgpu::Backends::GL
        }else{
            //If running natively, choose vulkan if on windows, otherwise choose a default
            if cfg!(target_os="windows"){
                wgpu::Backends::VULKAN
            }else{
                wgpu::Backends::PRIMARY
            }
        };

        //Instance
        //Validation and shader debugging turned enabled by default
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: backend,
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

        println!("SURFACE FORMAT: {:#?}",surface_format);

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


       

        //Buffers
        //Creates and initializes buffer data
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor{
            label: Some("my vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES3),
            usage: wgpu::BufferUsages::VERTEX
        });

        let num_vertices = VERTICES2.len() as u32;

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor{
            label: Some("my index buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX
        });

        let num_indices = INDICES.len() as u32;

        //Images
        //CPU side data
        let diffuse_bytes = include_bytes!("happy_tree.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        let dimensions = diffuse_image.dimensions();

        //GPU side data
        let texture_size = wgpu::Extent3d{
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor{
            label: Some("my diffuse texture"),
            size: texture_size,
            mip_level_count: 1, //For mipmapping
            sample_count: 1, //For multisampling
            dimension: wgpu::TextureDimension::D2, //2D texture
            format: wgpu::TextureFormat::Rgba8UnormSrgb, //
            //Will copy CPU side data to texture & wil bind it in shader
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        //Copy the texture data to gpu
        queue.write_texture(
            //Where to copy
            wgpu::ImageCopyTexture{
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                //Color,depth,stencil
                aspect: wgpu::TextureAspect::All,
            },
            //The data
            &diffuse_rgba,
            //Texture layout
            wgpu::ImageDataLayout{
                offset: 0,
                bytes_per_row: Some(4*dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size
        );

        //Create a texture view and sampler
        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor{
            label: Some("my sampler"),
            //Handles tex coords outside [0,1]^2
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,

            //When texel size is too small,too big
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        //Create depth texture
        let depth_texture_dimensions = wgpu::Extent3d{
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor{
            label: Some("my depth texture"),
            size: depth_texture_dimensions,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        //Bind group layout & bind group
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("my bind group"),
            entries: &[
                //Entry for texture view
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    //Frag shader has access to it
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture{
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None,
                },
                //Entry for texture sampler
                wgpu::BindGroupLayoutEntry{
                    binding: 1,
                    //Frag shader has access to it
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }
            ],
        });

        //Build the bind group here by attaching resources
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("texture bind group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry{
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
        });

         

        //Camera stuff
        let camera_controller = camera::CameraController::new(0.01);

        let camera = camera::Camera::new(
            (0.0,1.0,2.0).into(),
            (0.0,0.0,0.0).into(),
            cgmath::Vector3::unit_y(),
            config.width as f32 / config.height as f32,
            45.0,
            0.1,
            100.0,
        );

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("my camera buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            //Uniforms in shaders, will copy to often via queue operations
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("my camera bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer{
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("my camera bind group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 0,
                    resource: camera_buffer.as_entire_binding()
                }
            ],
        });

        //Instances

        let mut instances = vec![];
        for i in 0..INSTANCES_PER_ROW{
            for j in 0..INSTANCES_PER_ROW{
                let position = cgmath::Vector3 { x: i as f32, y: 0.0, z: j as f32 } - INSTANCE_DX;

                let rotation = if position.magnitude2() < 0.001 {
                    // this is needed so an object at (0, 0, 0) won't get scaled to zero
                    // as Quaternions can affect scale if they're not created correctly
                    cgmath::Quaternion::from_angle_z(cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                instances.push(Instance::new(position, rotation));
            }
        }

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("my instance buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        }); 


        //Pipelines

        //Shader modules first
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("my shader module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader5.wgsl").into())
        });

        //Pipeline layout (how uniform chunks of data are loaded into pipelines)
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("my render pipeline layout"),
            bind_group_layouts: 
            &[
                &texture_bind_group_layout,
                &camera_bind_group_layout
            ],
            push_constant_ranges: &[]
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("my render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    TextureVertex::layout(),
                    InstanceRaw::layout()
                ]
            },
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, //No culling rn
                //Requires: depth clip control feature
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                //Requires: conservative raster feature
                conservative: false
            },
            //For depth-testing
            depth_stencil: Some(wgpu::DepthStencilState{
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            //For MSAA type stuff
            multisample: MultisampleState{
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            fragment: Some(FragmentState{
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState{
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),
            //For multi-view rendering (to array of textures, like a dynamic cubemap or something?)
            multiview: None,
        });

        Self{
            window,
            surface,
            device,
            queue,
            config,
            size,
            bg_color: wgpu::Color{r: 1.0,g: 0.0,b: 0.0,a: 1.0},
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            camera_controller,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer,
            time: 0.0,
            depth_texture,
            depth_texture_view
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
            }
            _ => {}
        }
        if self.camera_controller.handle_input(event){
            return true
        }
        false
    }

    fn update(&mut self) {

        self.time+=0.01;

        //Update camera
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update(&self.camera);
        self.queue.write_buffer(&self.camera_buffer,0, bytemuck::cast_slice(&[self.camera_uniform]));

        //Update instances
        for instance in self.instances.iter_mut(){
            instance.rotation = if instance.position.magnitude2() < 0.001 {
                // this is needed so an object at (0, 0, 0) won't get scaled to zero
                // as Quaternions can affect scale if they're not created correctly
                cgmath::Quaternion::from_angle_z(cgmath::Deg(self.time))
            } else {
                cgmath::Quaternion::from_axis_angle(instance.position.normalize(), cgmath::Deg(45.0 + self.time))
            }
        }
        //Turn to raw then update gpu buffer
        let instances_raw = self.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        self.queue.write_buffer(&self.instance_buffer,0,bytemuck::cast_slice(&instances_raw));
        
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("my cmd encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("my render pass"),
                //This is @location(0) of render pipeline
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.bg_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment{
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations{
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            //Bind pipeline
            render_pass.set_pipeline(&self.render_pipeline);

            //Bind...well bind groups
            render_pass.set_bind_group(0,&self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

            //Bind resources 
            render_pass.set_vertex_buffer(0,self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));


            render_pass.set_index_buffer(self.index_buffer.slice(..),wgpu::IndexFormat::Uint16);

            //Issue commands
            //render_pass.draw(0..3, 0..1); (for vertex buffer)
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as u32);

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
        //window.window.request_inner_size(PhysicalSize::new(100, 400));
        
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

    println!("Exiting...");
}