// a cache that returns an object that implements a trait Render 

use std::{any::TypeId, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use lazy_static::lazy_static;
use wgpu::{util::DeviceExt, RenderPipeline};

use crate::{render_data::RenderData, render_pipeline::{DefaultPipeline, PIPELINE_CACHE}, texture::Texture, vertex::Vertex};

pub trait Renderable{
    fn choose_pipeline(&self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration)->Arc<RenderPipeline>;
    fn load_data(&self, device: &wgpu::Device, queue: &wgpu::Queue, bind_group_layout: &wgpu::BindGroupLayout)->RenderData;
    fn render(&self, render_pass: &mut wgpu::RenderPass, device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration){
        
        let pipeline = self.choose_pipeline(device, config);



        let data = self.load_data(device, queue, &pipeline.get_bind_group_layout(0));
        render_pass.set_bind_group(0, &data.bind_group, &[]);
        render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
        render_pass.set_index_buffer(data.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..data.num_indices, 0, 0..1);
    }
}

#[derive(Default)]
pub struct Renderables{
    pub renderables: Vec<Box<dyn Renderable + Send + Sync>>,
}

lazy_static!{
    pub static ref RENDERABLES: Mutex<Renderables> = Mutex::new(Renderables::default());
}

pub struct Polygon;

impl Renderable for Polygon{
    fn choose_pipeline(&self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration)->Arc<RenderPipeline>{
        PIPELINE_CACHE.lock().unwrap().get_pipeline::<DefaultPipeline>(device, config) // 2.
        // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
        // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // 3.
        // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
    
    fn load_data(&self, device: &wgpu::Device, queue: &wgpu::Queue, bind_group_layout: &wgpu::BindGroupLayout)->RenderData {
        const VERTICES: &[Vertex] = &[
            Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
        ];

        const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;
        let texture = Texture::from_bytes(device, queue, include_bytes!("happy-tree.png"), "texture").unwrap();   
        let bind_group = DefaultPipeline::create_bind_group(device, texture);
        RenderData{vertex_buffer, index_buffer, bind_group, num_indices}
    }
    // functions to load the data, but where to store them?
}