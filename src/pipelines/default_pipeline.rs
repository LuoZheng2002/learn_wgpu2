use std::any::TypeId;

use wgpu::{BindGroupLayout, RenderPipeline};

use crate::{my_pipeline::{MyPipeline, PipelineBuilder}, my_texture::MyTexture, render_context::{self, RenderContext}, render_passes::opauqe3d_render_pass::Opaque3DRenderPass, vertex::Vertex};

pub struct DefaultPipeline;

impl DefaultPipeline {
    fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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

    pub fn create_texture_bind_group(device: &wgpu::Device, texture: &MyTexture) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Self::create_texture_bind_group_layout(device),
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
    // bind groups like textures should be per-model
    // bind groups like instance buffers should be per-instance

    // we need an array of models, and for each model an array of instances

    // the drawable should be an instance with model and transform information

    // separate render context and game data (the drawable should only contain the model file location)

    // each model has its bind group of textures, ... etc.

    // skybox has a different pipeline

    // one model, different materials, different render passes, so must rebind information like rigs

    // but file path can no longer be the identifier for a model because there are several meshes

    // it is likely that different pipelines need different render passes, so for each pipeline we create a render pass

    // pipeline.render

    // model is an object instance



    pub fn create_bind_groups<'a>(
        render_context: &'a RenderContext,
        texture: &MyTexture
    ) -> Vec<&'a wgpu::BindGroup> {
        let texture_bind_group = Self::create_texture_bind_group(&render_context.device, texture);
        let camera_bind_group = &render_context.camera_bind_group;
        vec![texture_bind_group, camera_bind_group]
    }
}

impl PipelineBuilder for DefaultPipeline {
    fn build_pipeline(&self, render_context: &RenderContext) -> MyPipeline {
        let device = &render_context.device;
        let config = &render_context.config;

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&Self::create_texture_bind_group_layout(device), &render_context.camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("default.wgsl").into()),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: MyTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual, // 1.
                stencil: wgpu::StencilState::default(), // 2.
                bias: wgpu::DepthBiasState::default(),
            }), // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None,     // 6.
        });
        MyPipeline{
            pipeline: render_pipeline,
            render_pass_builder: TypeId::of::<Opaque3DRenderPass>(), // 1. Store the type ID of the pipeline builder for later use.
        }
    }
}
