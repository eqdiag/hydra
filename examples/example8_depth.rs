use hydra::{app::{App, EventHandler, Frame}, camera::{self, PerspectiveParams}, context::Context, pipeline::RenderPipelineBuilder, texture, vertex::{BasicInstanceData, ColoredVertex, TexturedVertex, VertexLayout}};
use image::GenericImageView;
use nalgebra_glm::{identity, quat_cast, rotate_y, to_quat, translation, two_pi, vec3};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Backends, ImageCopyTexture, ImageCopyTextureBase, IndexFormat, ShaderModule, ShaderSource, VertexBufferLayout};
use winit::{event::{ElementState, MouseButton}, keyboard::KeyCode::*, window};

const VERTICES: &[TexturedVertex] = &[
    TexturedVertex { position: [-0.0868241, 0.49240386, 0.0], uv: [0.4131759, 0.00759614], }, 
    TexturedVertex { position: [-0.49513406, 0.06958647, 0.0], uv: [0.0048659444, 0.43041354], }, 
    TexturedVertex { position: [-0.21918549, -0.44939706, 0.0], uv: [0.28081453, 0.949397], }, 
    TexturedVertex { position: [0.35966998, -0.3473291, 0.0], uv: [0.85967, 0.84732914], }, 
    TexturedVertex { position: [0.44147372, 0.2347359, 0.0], uv: [0.9414737, 0.2652641], }, 
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

const NUM_INSTANCES: u32 = 10;

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MatrixUniform{
    matrix: [[f32;4];4],
}

impl MatrixUniform{
    pub fn new() -> Self{
        MatrixUniform{
            matrix: nalgebra_glm::Mat4::identity().into()
        }
    }
}

struct Instance{
    position: nalgebra_glm::Vec3,
    rotation: nalgebra_glm::Quat
}


impl Instance{
    pub fn to_matrix(&self) -> MatrixUniform{
        MatrixUniform{
            matrix: (translation(&self.position) * quat_cast(&self.rotation)).into()
        }
    }
}


struct State{
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    texture: texture::Texture,
    texture_bind_group: wgpu::BindGroup,

    depth_texture: wgpu::Texture,
    depth_texture_view: wgpu::TextureView,

    //matrix stuff
    camera: camera::Camera,
    camera_controller: camera::FlyCameraController,
    matrix_bind_group: wgpu::BindGroup,

    //cpu side 4x4 matrix data
    cpu_matrix_uniform: MatrixUniform,
    //gpu side matrix data
    gpu_matrix_uniform: wgpu::Buffer,

    //instance stuff
    instances: Vec<Instance>,
    cpu_instance_data: Vec<MatrixUniform>,
    gpu_instance_data: wgpu::Buffer,

    pub t: f32
}

fn init(_app: &App<State>,ctx: &Context) -> State{


    //create buffers
    let vertex_buffer = ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("my vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    let index_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("my index buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    //uniform buffers

    let camera = camera::Camera::new(camera::ProjectionMatrix::Perspective(PerspectiveParams{
        aspect: ctx.config.width as f32 / ctx.config.height as f32,
        fovy: 45.0,
        near: 0.1,
        far: 100.0
    }));

    let camera_controller = camera::FlyCameraController::default();

    let mut cpu_matrix_uniform = MatrixUniform::new();

    cpu_matrix_uniform.matrix = camera.get_view_proj_matrix().into();


    let mut instances = vec![];
    for i in 0..NUM_INSTANCES{
        let angle = two_pi::<f32>() * (i as f32 / NUM_INSTANCES as f32);
        instances.push(Instance{
            position: vec3(0.1 * (i as f32), 0.0, -(i as f32)),
            rotation: to_quat(&rotate_y(&identity(),angle)),
        })
    }

    let cpu_instance_data = instances.iter().map(Instance::to_matrix).collect::<Vec<_>>();
    let gpu_instance_data = ctx.device.create_buffer_init(&BufferInitDescriptor{
        label: Some("gpu instance data"),
        contents: bytemuck::cast_slice(&cpu_instance_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    let gpu_matrix_uniform = ctx.device.create_buffer_init(&BufferInitDescriptor{
        label: Some("my gpu matrix buffer"),
        contents: bytemuck::cast_slice(&[cpu_matrix_uniform]),
        //using as uniform in shaders + will copy cpu-side data to it
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    //create images & textures

    //simple diffuse texture
    let image_bytes = include_bytes!("../assets/happy_tree.png");
    let texture = texture::Texture::from_bytes(ctx, image_bytes).unwrap();

    //depth texture    
    //same as swapchain texture
    let depth_texture_size = wgpu::Extent3d{
        width: ctx.size.width,
        height: ctx.size.height,
        depth_or_array_layers:1,
    };

    let depth_texture = ctx.device.create_texture(&wgpu::TextureDescriptor{
        label: Some("my depth texture"),
        size: depth_texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        //just like a color target, is an output of a pipeline
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[]
    });

    let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    //create samplers
    let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor{
        label: Some("my sampler"),
        //when u,v roll over edge
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    //bind group layout for textures
    let texture_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
        label: Some("my bind group layout"),
        entries: &[
            //texture view entry
            wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            //texture sampler entry
            wgpu::BindGroupLayoutEntry{
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let texture_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor{
        label: Some("my bind group"),
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry{
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry{
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            }
        ],
    });


    let matrix_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
        label: Some("my matrix bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
    });

    let matrix_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor{
        label: Some("my matrix bind group"),
        layout: &matrix_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry{
                binding: 0,
                resource: wgpu::BindingResource::Buffer(gpu_matrix_uniform.as_entire_buffer_binding()),
            }
        ],
    });
    


    //pipeline layout
    let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("my pipeline layout"),
        //for binding buffers,textures
        bind_group_layouts: &[
            &texture_bind_group_layout,
            &matrix_bind_group_layout
        ],
        //for pushing uniform data via commands (small data)
        push_constant_ranges: &[],
    });


    let color_target = wgpu::ColorTargetState{
        format: ctx.config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    };

    let pipeline = RenderPipelineBuilder::new(ctx)
        .with_shaders(ShaderSource::Wgsl(include_str!("../assets/example7_shader.wgsl").into()), "vs_main", "fs_main")
        .with_culling(None, wgpu::FrontFace::Ccw)
        .with_layout(pipeline_layout)
        .add_vertex_buffer_layout(TexturedVertex::layout())
        .add_vertex_buffer_layout(BasicInstanceData::layout())
        .add_color_target_state(color_target)
        .with_depth_stencil_state(wgpu::DepthStencilState{
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        })
        .build();
        

    State{
        pipeline,
        vertex_buffer,
        index_buffer,
        texture,
        texture_bind_group,
        depth_texture,
        depth_texture_view,
        camera,
        camera_controller,
        matrix_bind_group,
        cpu_matrix_uniform,
        gpu_matrix_uniform,
        instances,
        cpu_instance_data,
        gpu_instance_data,
        t: 0.0
    }
}



fn update(state: &mut State,ctx: &Context){

    state.t+=0.001;

    //update camera with controller
    state.camera_controller.update_camera(&mut state.camera);

    //update cpu camera buffer
    state.cpu_matrix_uniform.matrix = state.camera.get_view_proj_matrix().into();

    //update gpu camera buffer
    ctx.queue.write_buffer(&state.gpu_matrix_uniform, 0, bytemuck::cast_slice(&[state.cpu_matrix_uniform]));

    //update instances
    for i in 0..NUM_INSTANCES{
        let angle = two_pi::<f32>() * ((i as f32 / NUM_INSTANCES as f32) + state.t);
        state.instances[i as usize].rotation = to_quat(&rotate_y(&identity(),angle))
    }
    //update cpu side buffer
    state.cpu_instance_data = state.instances.iter().map(Instance::to_matrix).collect::<Vec<_>>();
    //update gpu side instance vertex buffer
    ctx.queue.write_buffer(&state.gpu_instance_data, 0,bytemuck::cast_slice(&state.cpu_instance_data));
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment{
                view: &state.depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        //bind pipeline
        render_pass.set_pipeline(&state.pipeline);

        //bind resources (buffers,images)

        //buffers
        render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(state.index_buffer.slice(..),IndexFormat::Uint16);

        //set instance vertex buffer
        render_pass.set_vertex_buffer(1,state.gpu_instance_data.slice(..));

        //bind groups
        render_pass.set_bind_group(0,&state.texture_bind_group, &[]);
        render_pass.set_bind_group(1,&state.matrix_bind_group,&[]);

        
        //make render calls
        render_pass.draw_indexed(0..(INDICES.len() as u32),0, 0..NUM_INSTANCES);
    }

    ctx.queue.submit(std::iter::once(encoder.finish()));
    frame.present();

}

fn resize(state: &mut State,ctx: &Context,width: u32,height: u32,event_handler: EventHandler){
    if width > 0 && height > 0{
        state.camera.update_to_perspective(PerspectiveParams{
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            near: 0.1,
            far: 100.0,
        })
    }

    //recreate depth texture and view
    state.depth_texture = ctx.device.create_texture(&wgpu::TextureDescriptor{
        label: Some("my depth texture"),
        size: wgpu::Extent3d{
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        //just like a color target, is an output of a pipeline
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[]
    });
    state.depth_texture_view = state.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
}

fn key_input(state: &mut State,key: hydra::app::Key,key_state: ElementState,event_handler: EventHandler){
    state.camera_controller.on_key_fn(key, key_state);
    match key{
        Escape => event_handler.exit(),
        _ => {}
    }
}

fn mouse_move(state: &mut State,delta: (f32,f32),event_handler: EventHandler){
    state.camera_controller.on_mouse_move_fn(delta);
}

fn mouse_input(state: &mut State,mouse_button: MouseButton,button_state: ElementState,event_handler: EventHandler){
    state.camera_controller.on_mouse_input_fn(button_state, mouse_button);
}


fn main(){
    App::new(init)
    .update(update)
    .render(render)
    .on_window_resize(resize)
    .on_key(key_input)
    .on_mouse_move(mouse_move)
    .on_mouse_input(mouse_input)
    .with_title("example8_depth".to_string())
    .run();
}