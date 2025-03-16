use image::GenericImageView;

use crate::render_context::RenderContext;



pub struct CubeTexture {
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
    pub view: wgpu::TextureView,
}

impl CubeTexture {

    pub fn from_files(render_context: &RenderContext,
        directory: &str, label: &str) -> Result<Self, image::ImageError> {
            let device = &render_context.device;
            let queue = &render_context.queue;
            let mut dimensions: Option<(u32, u32)> = None;
            let mut texture: Option<wgpu::Texture> = None;
            let file_names = ["px.png", "nx.png", "py.png", "ny.png", "pz.png", "nz.png"]
                .iter()
                .map(|file_name| format!("{}/{}", directory, file_name))
                .collect::<Vec<_>>();
            for (i, file_name) in file_names.iter().enumerate() {
                let img = image::open(file_name)?;
                let rgba = img.to_rgba8();
                if dimensions.is_none() {
                    dimensions = Some(rgba.dimensions());
                    assert!(texture.is_none());
                    texture = Some(device.create_texture(&wgpu::TextureDescriptor {
                        label: Some(label),
                        size: wgpu::Extent3d {
                            width: dimensions.unwrap().0,
                            height: dimensions.unwrap().1,
                            depth_or_array_layers: 6,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                        view_formats: &[],
                    }));
                }
                let dimensions = dimensions.unwrap();
                println!("Writing texture data for face {}", i);
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        aspect: wgpu::TextureAspect::All,
                        texture: texture.as_ref().unwrap(),
                        mip_level: 0,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: i as u32,
                        },
                    },
                    &rgba,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * dimensions.0),
                        rows_per_image: Some(dimensions.1),
                    },
                    wgpu::Extent3d {
                        width: dimensions.0,
                        height: dimensions.1,
                        depth_or_array_layers: 1, // Only write one layer at a time
                    },
                );
            }
            let texture = texture.unwrap();    
            let view = texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some("Cube Texture View"),
                dimension: Some(wgpu::TextureViewDimension::Cube),
                array_layer_count: Some(6),
                ..Default::default()
            });
    
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("Cube Texture Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });
            Ok(Self {
                texture,
                sampler,
                view,
            })
    }
}