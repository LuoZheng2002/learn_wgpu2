use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use wgpu::{Device, RenderPipeline, SurfaceConfiguration};

use crate::{
    render_context::{self, RenderContext},
    texture::Texture,
    vertex::Vertex,
};
use lazy_static::lazy_static;

/// Trait that must be implemented by custom pipeline types.
pub trait ToPipeline {
    fn create_pipeline(render_context: &RenderContext) -> RenderPipeline;
}

/// Cache structure that holds the pipelines.
#[derive(Default)]
pub struct PipelineCache {
    pipelines: HashMap<TypeId, Arc<RenderPipeline>>,
}

impl PipelineCache {
    /// Generic method to get a pipeline or create it if it's not in the cache.
    pub fn get_pipeline<T>(&mut self, render_context: &RenderContext) -> Arc<RenderPipeline>
    where
        T: ToPipeline + 'static,
    {
        let type_id = TypeId::of::<T>();
        self.pipelines
            .entry(type_id)
            .or_insert_with(|| {
                println!("Creating pipeline");
                let pipeline = T::create_pipeline(render_context);
                Arc::new(pipeline)
            })
            .clone()
    }
}

lazy_static! {
    /// Global cache that holds all the pipelines.
    pub static ref PIPELINE_CACHE: Mutex<PipelineCache> = Mutex::new(PipelineCache::default());
}
