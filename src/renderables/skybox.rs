use std::{collections::HashMap, sync::{Arc, Mutex}};

use lazy_static::lazy_static;
use wgpu::{RenderPipeline, util::DeviceExt};

use crate::{
    cube_texture::CubeTexture, pipelines::{default_pipeline::DefaultPipeline, skybox_pipeline::SkyboxPipeline}, render_context::RenderContext, render_passes::RenderPassType, render_pipeline::PipelineCache, renderable::Renderable, texture::Texture, vertex::Vertex
};

pub struct Skybox{
    directory: String,
    texture_bind_group: Option<wgpu::BindGroup>,
}
impl Skybox{
    pub fn new(directory: String) -> Self {
        Self {
            directory,
            texture_bind_group: None,
        }
    }
}



impl Renderable for Skybox {
    fn choose_pipeline(&self, render_context: &RenderContext, pipeline_cache: &mut PipelineCache) -> Arc<(RenderPipeline, RenderPassType)> {
        pipeline_cache.get_pipeline::<SkyboxPipeline>(render_context) // 2.
        // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
        // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // 3.
        // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
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
        let cube_texture = CUBE_TEXTURES.lock().unwrap().entry(self.directory.clone()).or_insert_with(|| {
            let cube_texture = CubeTexture::from_files(render_context, &self.directory, "Cube Texture").unwrap();
            Arc::new(cube_texture)
        }).clone();
        let bind_groups: Vec<&'a wgpu::BindGroup> = SkyboxPipeline::create_bind_groups(render_context, &cube_texture, &mut self.texture_bind_group);
        bind_groups
    }
    fn get_num_indices(&self) -> u32 {
        INDICES.len() as u32
    }
}


lazy_static!{
    #[rustfmt::skip]
    static ref VERTICES: &'static [Vertex] = &[
        // Front face (Z = -1)
        Vertex {position: [-1.0, -1.0, 1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [-1.0, 1.0, 1.0], tex_coords: [0.0, 0.0]},            
        Vertex {position: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, -1.0, 1.0], tex_coords: [0.0, 0.0]},
        // Back face (Z = 1)
        Vertex {position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 0.0]},    
        Vertex {position: [1.0, -1.0, -1.0], tex_coords: [0.0, 0.0]},               
        Vertex {position: [1.0, 1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [-1.0, 1.0, -1.0], tex_coords: [0.0, 0.0]},     
        // Left face (X = -1)
        Vertex {position: [-1.0, -1.0, 1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [-1.0, 1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [-1.0, 1.0, 1.0], tex_coords: [0.0, 0.0]},
        // Right face (X = 1)
        Vertex {position: [1.0, -1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, -1.0, 1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, 1.0, -1.0], tex_coords: [0.0, 0.0]},
        // Top face (Y = 1)
        Vertex {position: [-1.0, 1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, 1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0]},            
        Vertex {position: [-1.0, 1.0, 1.0], tex_coords: [0.0, 0.0]},
        // Bottom face (Y = -1)
        Vertex {position: [-1.0, -1.0, 1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, -1.0, 1.0], tex_coords: [0.0, 0.0]},            
        Vertex {position: [1.0, -1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 0.0]},
    ];
    #[rustfmt::skip]
    static ref INDICES: &'static [u16] = &[
        0, 1, 2, 2, 3, 0, // Front
        4, 5, 6, 6, 7, 4, // Back
        8, 9, 10, 10, 11, 8, // Left
        12, 13, 14, 14, 15, 12, // Right
        16, 17, 18, 18, 19, 16, // Top
        20, 21, 22, 22, 23, 20, // Bottom
    ];
    static ref VERTEX_BUFFER: Mutex<Option<Arc<wgpu::Buffer>>> = Mutex::new(None);
    static ref INDEX_BUFFER: Mutex<Option<Arc<wgpu::Buffer>>> = Mutex::new(None);
    static ref CUBE_TEXTURES: Mutex<HashMap<String, Arc<CubeTexture>>> = Mutex::new(HashMap::new());
}