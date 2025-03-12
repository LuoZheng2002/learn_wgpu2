use std::{any::TypeId, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use wgpu::{Device, RenderPipeline, SurfaceConfiguration};

use crate::{texture::Texture, vertex::Vertex};
use lazy_static::lazy_static;

/// Trait that must be implemented by custom pipeline types.
pub trait ToPipeline {
    fn create_pipeline(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline;
    fn get_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout;
}

/// Cache structure that holds the pipelines.
#[derive(Default)]
pub struct PipelineCache {
    pipelines: HashMap<TypeId, Arc<RenderPipeline>>,
}

impl PipelineCache{
    /// Generic method to get a pipeline or create it if it's not in the cache.
    pub fn get_pipeline<T>(&mut self, device: &Device, config: &SurfaceConfiguration) -> Arc<RenderPipeline>
    where
        T: ToPipeline + 'static,
    {
        // Check if the pipeline already exists in the cache.
        if let Some(pipeline) = self.pipelines.get(&TypeId::of::<T>()) {
            pipeline.clone()
        } else {
            // If not, create the pipeline and insert it into the cache.
            let pipeline = T::create_pipeline(device, config);
            let pipeline = Arc::new(pipeline);
            self.pipelines.insert(TypeId::of::<T>(), pipeline.clone());
            pipeline
        }
    }
}

lazy_static! {
    /// Global cache that holds all the pipelines.
    pub static ref PIPELINE_CACHE: Mutex<PipelineCache> = Mutex::new(PipelineCache::default());
}

pub struct DefaultPipeline;


impl DefaultPipeline{
    pub fn create_bind_group(device: &wgpu::Device, texture: Texture)->wgpu::BindGroup{
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Self::get_bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        })
    }
}

impl ToPipeline for DefaultPipeline{
    fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }
    fn create_pipeline(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline {
        let texture_bind_group_layout = Self::get_bind_group_layout(device);
            
      
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
    
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: &[Vertex::desc()],   // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None,     // 6.
        });
        render_pipeline
    }
    
    
}