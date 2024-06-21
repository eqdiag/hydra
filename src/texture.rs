use image::GenericImageView;
use wgpu::ImageCopyTexture;

//Contains a texture,a texture view, a sampler, and a bind group for that texture
pub struct TextureBundle{
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup
}

impl TextureBundle{
    pub fn from_bytes(context: &crate::context::Context, bytes: &[u8],bind_group_layout: &wgpu::BindGroupLayout) -> Result<Self,image::ImageError>{
        let image = image::load_from_memory(bytes)?;
        //convert it
        let image_rgba = image.to_rgba8();
        let image_size = image.dimensions();

        //texture
        let texture_size = wgpu::Extent3d{
            width: image_size.0,
            height: image_size.1,
            depth_or_array_layers: 1,
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor{
            label: Some("my texture"),
            size: texture_size,
            //no mimpapping rn
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            //will bind in bind group, will also copy to it from cpu image
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            //swizzle for textures
            view_formats: &[],
        });

        //copy it to gpu memory
        context.queue.write_texture(
            ImageCopyTexture{
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4*image_size.0),
                rows_per_image: Some(image_size.1),
            },
            texture_size
        );

        //texture view
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor{
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

        let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("my bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry{
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
        });

        Ok(TextureBundle{
            texture,
            view,
            sampler,
            bind_group
        })

    }
}