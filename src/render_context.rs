use std::{any::TypeId, collections::{BTreeSet, HashMap, HashSet}, sync::Arc};

use tokio::runtime::Runtime;
use wgpu::{Surface, util::DeviceExt};
use winit::window::Window;

use crate::{
    camera_uniform::CameraUniform, my_render_pass::RENDER_PASS_BUILDERS, my_texture::MyTexture, renderable::Renderable, state::State
};

pub struct RenderContext {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub camera_buffer: wgpu::Buffer,
    // most pipelines will use this
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_bind_group: wgpu::BindGroup,
    pub depth_texture: MyTexture,
}

impl RenderContext {
    pub fn new(window: Window) -> Self {
        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::DEBUG,
            ..Default::default()
        });
        let window = Arc::new(window);
        let surface: Surface<'static> = instance.create_surface(window.clone()).unwrap();

        // tokio runtime for converting async functions to blocking sync functions
        let runtime = Runtime::new().unwrap();
        // adapter is the gpu information
        let adapter = runtime
            .block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();
        let (device, queue) = runtime
            .block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: Default::default(),
                },
                None, // Trace path
            ))
            .unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // camera stuff
        // let camera = Camera {
        //     // position the camera 1 unit up and 2 units back
        //     // +z is out of the screen
        //     eye: (0.0, 1.0, 2.0).into(),
        //     // have it look at the origin
        //     target: (0.0, 0.0, 0.0).into(),
        //     // which way is "up"
        //     up: cgmath::Vector3::unit_y(),
        //     aspect: config.width as f32 / config.height as f32,
        //     fovy: 45.0,
        //     znear: 0.1,
        //     zfar: 100.0,
        // };

        // let mut camera_uniform = CameraUniform::new();
        // camera_uniform.update_view_proj(&camera);
        let camera_uniform = CameraUniform::default();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let depth_texture = MyTexture::create_depth_texture(&device, &config, "depth texture");

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("view_bind_group_layout"),
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            camera_buffer,
            depth_texture,
            camera_bind_group_layout,
            camera_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        self.depth_texture = MyTexture::create_depth_texture(&self.device, &self.config, "depth texture");
    }

    pub fn render(&mut self, state: &mut State) -> Result<(), wgpu::SurfaceError> {
        // get render target
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // update camera transform
        let aspect = self.config.width as f32 / self.config.height as f32;
        let camera_uniform = CameraUniform::new(&state.camera, aspect, true);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );



        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        
        // Begin render passes
        let mut renderable_refs: HashMap<TypeId, Vec<&mut dyn Renderable>> = HashMap::new();
        for renderable in state.renderables.iter_mut().map(|renderable|renderable){
            let render_pass_type = renderable.get_render_pass_builder(self);
            let renderable_ref = renderable.as_mut();            
            renderable_refs.entry(render_pass_type).or_insert(vec![]).push(renderable_ref);
        }
        
        for (render_pass_type, render_pass_builder) in &*RENDER_PASS_BUILDERS {
            if !renderable_refs.contains_key(&render_pass_type) {
                // if the render pass type is not in the renderable_refs, we skip it
                continue;
            }
            let mut render_pass = render_pass_builder.create_render_pass(&mut encoder,&view, &self.depth_texture.view);
            let renderables = renderable_refs.get_mut(&render_pass_type).unwrap();
            for renderable in renderables {
                renderable.render(&mut render_pass, self);
            }
            renderable_refs.remove(&render_pass_type); // remove the entry after using it
        }
        // check if there is any render pass type that is not in RENDER_PASS_BUILDERS
        assert!(renderable_refs.is_empty(), "There are render pass types that are not in RENDER_PASS_BUILDERS");
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
