use std::{any::TypeId, collections::HashMap, sync::{Arc, Mutex}};

use lazy_static::lazy_static;
use wgpu::util::DeviceExt;

use crate::{
    pipelines::default_pipeline::DefaultPipeline, render_context::RenderContext,  renderable::Renderable, my_texture::{MyTexture, TextureSource}, vertex::Vertex
};

pub struct Cube{
    texture_file_path: String,
}
impl Cube{
    pub fn new(texture_file_path: String) -> Self {
        Self {
            texture_file_path,
        }
    }
}


impl Renderable for Cube {
    fn choose_pipeline(&self) -> TypeId {
        TypeId::of::<DefaultPipeline>()
    }
    fn get_vertex_buffer(&self, render_context: &RenderContext) -> Arc<wgpu::Buffer> {
        VERTEX_BUFFER.lock().unwrap().get_or_insert_with(||{
            let vertex_buffer = render_context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });
            Arc::new(vertex_buffer)
        }).clone()
    }
    fn get_index_buffer(&self, render_context: &RenderContext) -> Arc<wgpu::Buffer> {
        INDEX_BUFFER.lock().unwrap().get_or_insert_with(||{
            let index_buffer = render_context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
            Arc::new(index_buffer)
        }).clone()
    }
    fn get_bind_groups<'a>(&'a mut self, render_context: &'a RenderContext) -> Vec<&'a wgpu::BindGroup> {
        let texture = TEXTURES.lock().unwrap().entry(self.texture_file_path.clone()).or_insert_with(|| {
            let texture = MyTexture::load(TextureSource::FilePath(self.texture_file_path.clone()), render_context, Some("cube texture")).unwrap();
            Arc::new(texture)
        }).clone();
        let bind_groups: Vec<&'a wgpu::BindGroup> = DefaultPipeline::create_bind_groups(render_context, &texture);
        bind_groups
    }
    fn get_num_indices(&self) -> u32 {
        INDICES.len() as u32
    }
    // functions to load the data, but where to store them?
}

lazy_static!{
    #[rustfmt::skip]
    static ref VERTICES: &'static [Vertex] = &[
        // Front face (Z = 1)
        Vertex {position: [-1.0, -1.0, 1.0],tex_coords: [1./4., 2./3.],},
        Vertex {position: [1.0, -1.0, 1.0],tex_coords: [2./4., 2./3.],},
        Vertex {position: [1.0, 1.0, 1.0],tex_coords: [2./4., 1./3.],},
        Vertex {position: [-1.0, 1.0, 1.0],tex_coords: [1./4., 1./3.],},
        // Back face (Z = -1)
        Vertex {position: [-1.0, -1.0, -1.0],tex_coords: [3./4., 2./3.],},
        Vertex {position: [1.0, -1.0, -1.0],tex_coords: [1.0, 2./3.],},
        Vertex {position: [1.0, 1.0, -1.0],tex_coords: [1.0, 1./3.],},
        Vertex {position: [-1.0, 1.0, -1.0],tex_coords: [3./4., 1./3.],},
        // Left face (X = -1)
        Vertex {position: [-1.0, -1.0, -1.0],tex_coords: [0.0, 2./3.],},
        Vertex {position: [-1.0, -1.0, 1.0],tex_coords: [1./4., 2./3.],},
        Vertex {position: [-1.0, 1.0, 1.0],tex_coords: [1./4., 1./3.],},
        Vertex {position: [-1.0, 1.0, -1.0],tex_coords: [0.0, 1./3.],},
        // Right face (X = 1)
        Vertex {position: [1.0, -1.0, -1.0],tex_coords: [0.0, 0.0],},
        Vertex {position: [1.0, 1.0, -1.0],tex_coords: [0.0, 1.0],},
        Vertex {position: [1.0, 1.0, 1.0],tex_coords: [1.0, 1.0],},
        Vertex {position: [1.0, -1.0, 1.0],tex_coords: [1.0, 0.0],},
        // Top face (Y = 1)
        Vertex {position: [-1.0, 1.0, -1.0],tex_coords: [0.0, 0.0],},
        Vertex {position: [-1.0, 1.0, 1.0],tex_coords: [0.0, 1.0],},
        Vertex {position: [1.0, 1.0, 1.0],tex_coords: [1.0, 1.0],},
        Vertex {position: [1.0, 1.0, -1.0],tex_coords: [1.0, 0.0],},
        // Bottom face (Y = -1)
        Vertex {position: [-1.0, -1.0, -1.0],tex_coords: [0.0, 0.0],},
        Vertex {position: [1.0, -1.0, -1.0],tex_coords: [1.0, 0.0],},
        Vertex {position: [1.0, -1.0, 1.0],tex_coords: [1.0, 1.0],},
        Vertex {position: [-1.0, -1.0, 1.0],tex_coords: [0.0, 1.0],},
    ];
    #[rustfmt::skip]
    static ref INDICES: &'static [u16] = &[
        0, 1, 2, 2, 3, 0, // Front
        4, 6, 5, 6, 4, 7, // Back
        8, 9, 10, 10, 11, 8, // Left
        12, 13, 14, 14, 15, 12, // Right
        16, 17, 18, 18, 19, 16, // Top
        20, 21, 22, 22, 23, 20, // Bottom
    ];
    static ref VERTEX_BUFFER: Mutex<Option<Arc<wgpu::Buffer>>> = Mutex::new(None);
    static ref INDEX_BUFFER: Mutex<Option<Arc<wgpu::Buffer>>> = Mutex::new(None);
    static ref TEXTURES: Mutex<HashMap<String, Arc<MyTexture>>> = Mutex::new(HashMap::new());
}