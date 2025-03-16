// a cache that returns an object that implements a trait Render

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use wgpu::RenderPipeline;

use crate::render_context::RenderContext;

use crate::get_type::GetType;
use crate::render_passes::RenderPassType;
use crate::render_pipeline::PipelineCache;

pub trait Renderable: GetType {
    fn choose_pipeline(&self, render_context: &RenderContext, pipeline_cache: &mut PipelineCache) -> Arc<(RenderPipeline, RenderPassType)>;
    fn get_vertex_buffer(&self, render_context: &RenderContext) -> Arc<wgpu::Buffer>;
    fn get_index_buffer(&self, render_context: &RenderContext) -> Arc<wgpu::Buffer>;
    fn get_bind_groups<'a>(&'a mut self, render_context: &'a RenderContext) -> Vec<&'a wgpu::BindGroup>;
    fn get_num_indices(&self) -> u32;
    fn render(&mut self, render_pass: &mut wgpu::RenderPass,
         render_context: &RenderContext,
         pipeline_cache: &mut PipelineCache
    ){
        let pipeline = &self.choose_pipeline(render_context, pipeline_cache).0;
        render_pass.set_pipeline(pipeline);
        let vertex_buffer = self.get_vertex_buffer(render_context);
        let index_buffer = self.get_index_buffer(render_context);
        let num_indices = self.get_num_indices();
        let bind_groups = self.get_bind_groups(render_context);        
        for (i, bind_group) in bind_groups.iter().enumerate() {
            render_pass.set_bind_group(i as u32, *bind_group, &[]);
        }
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }
    fn get_render_pass_type(&self, render_context: &mut RenderContext, pipeline_cache: &mut PipelineCache) -> RenderPassType {
        self.choose_pipeline(render_context, pipeline_cache).1.clone()
    }
}

// lazy_static! {
//     pub static ref RENDERABLES: Mutex<Vec<Box<dyn Renderable + Send + Sync>>> =
//         Mutex::new(Default::default());
// }
