// a cache that returns an object that implements a trait Render

use std::{
    any::TypeId,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use wgpu::{RenderPipeline, util::DeviceExt};

use crate::{
    render_context::RenderContext,
    render_data::{RENDER_DATA_CACHE, RenderData},
    render_pipeline::{DefaultPipeline, PIPELINE_CACHE},
    texture::Texture,
    vertex::Vertex,
};

use crate::get_type::GetType;

pub trait Renderable: GetType {
    fn choose_pipeline(&self, render_context: &RenderContext) -> Arc<RenderPipeline>;
    fn load_data(&self, render_context: &RenderContext) -> RenderData;
    fn render(&self, render_pass: &mut wgpu::RenderPass, render_context: &RenderContext)
    // where Self: Sized + 'static
    {
        let pipeline = self.choose_pipeline(render_context);
        let mut render_data_cache = RENDER_DATA_CACHE.lock().unwrap();
        let data = render_data_cache.get_data(self, render_context);
        render_pass.set_pipeline(&pipeline);
        for (i, bind_group) in data.bind_groups.iter().enumerate() {
            render_pass.set_bind_group(i as u32, bind_group, &[]);
        }
        render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
        render_pass.set_index_buffer(data.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..data.num_indices, 0, 0..1);
    }
}

lazy_static! {
    pub static ref RENDERABLES: Mutex<Vec<Box<dyn Renderable + Send + Sync>>> =
        Mutex::new(Default::default());
}
