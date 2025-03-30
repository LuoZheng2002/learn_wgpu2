use std::{any::TypeId, collections::HashMap, sync::{Arc, Mutex}};

use lazy_static::lazy_static;
use wgpu::{RenderPipeline, util::DeviceExt};

use crate::{
    pipelines::{default_pipeline::DefaultPipeline, ui_pipeline::UIPipeline}, render_context::RenderContext, renderable::Renderable, my_texture::{MyTexture, TextureSource}, vertex::Vertex
};



pub struct UI{
    texture_source: TextureSource,
    texture_bind_group: Option<wgpu::BindGroup>,
}
impl UI{
    pub fn new(ui_source: TextureSource) -> Self {
        Self {
            texture_source: ui_source,
            texture_bind_group: None,
        }
    }
}

impl Renderable for UI {
    fn choose_pipeline(&self) -> TypeId {
        TypeId::of::<UIPipeline>() // This will choose the UIPipeline for rendering the UI.
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
        let texture = TEXTURES.lock().unwrap().entry(self.texture_source.clone()).or_insert_with(|| {
            let texture = MyTexture::load(self.texture_source.clone(), render_context, Some("ui texture")).unwrap();
            Arc::new(texture)
        }).clone();
        let bind_groups: Vec<&'a wgpu::BindGroup> = DefaultPipeline::create_bind_groups(render_context, &texture, &mut self.texture_bind_group);
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
        Vertex {position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 0.0]},
        Vertex {position: [1.0, -1.0, 0.0], tex_coords: [1.0, 0.0]},
        Vertex {position: [1.0, 1.0, 0.0], tex_coords: [1.0, 1.0]},
        Vertex {position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 1.0]},
    ];
    #[rustfmt::skip]
    static ref INDICES: &'static [u16] = &[
        0, 1, 2, 2, 3, 0
    ];
    static ref VERTEX_BUFFER: Mutex<Option<Arc<wgpu::Buffer>>> = Mutex::new(None);
    static ref INDEX_BUFFER: Mutex<Option<Arc<wgpu::Buffer>>> = Mutex::new(None);
    static ref TEXTURES: Mutex<HashMap<TextureSource, Arc<MyTexture>>> = Mutex::new(HashMap::new());
}