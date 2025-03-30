// a cache that returns an object that implements a trait Render

use std::any::TypeId;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use wgpu::RenderPipeline;

use crate::cache::{self, CacheValue, CACHE};
use crate::my_pipeline::{MyPipeline, PIPELINE_BUILDERS};
use crate::render_context::{self, RenderContext};

fn get_pipeline_from_cache(pipeline_type: TypeId, render_context: &RenderContext)->Arc<CacheValue>{
    CACHE.get_with(cache::CacheKey::Pipeline(pipeline_type), || {
        let pipeline = PIPELINE_BUILDERS.get(&pipeline_type).expect("Pipeline builder not found")
            .build_pipeline(render_context);
        Arc::new(CacheValue::Pipeline(pipeline))
    })
}

fn unpack_pipeline(pipeline: &Arc<CacheValue>) -> &MyPipeline {
    if let CacheValue::Pipeline(my_pipeline) = pipeline.as_ref() {
        my_pipeline
    } else {
        panic!("Failed to unpack pipeline from cache");
    }
}

pub trait Renderable {
    fn choose_pipeline(&self) -> TypeId;
    fn get_vertex_buffer(&self, render_context: &RenderContext) -> Arc<wgpu::Buffer>;
    fn get_index_buffer(&self, render_context: &RenderContext) -> Arc<wgpu::Buffer>;
    fn get_bind_groups<'a>(&'a mut self, render_context: &'a RenderContext) -> Vec<&'a wgpu::BindGroup>;
    fn get_num_indices(&self) -> u32;
    fn render(&mut self, render_pass: &mut wgpu::RenderPass,
         render_context: &RenderContext,
    ){
        let pipeline_type = self.choose_pipeline();
        let pipeline = get_pipeline_from_cache(pipeline_type, render_context);
        let pipeline = unpack_pipeline(&pipeline);
        render_pass.set_pipeline(&pipeline.pipeline);
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
    fn get_render_pass_builder(&self, render_context: &RenderContext) -> TypeId {
        let pipeline_type = self.choose_pipeline();
        let pipeline = get_pipeline_from_cache(pipeline_type, render_context); // Assuming you have a default RenderContext for this example
        let pipeline = unpack_pipeline(&pipeline);
        pipeline.render_pass_builder
    }
}

// lazy_static! {
//     pub static ref RENDERABLES: Mutex<Vec<Box<dyn Renderable + Send + Sync>>> =
//         Mutex::new(Default::default());
// }
