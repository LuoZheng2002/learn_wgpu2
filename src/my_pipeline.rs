// renderables rely on pipelines, pipeline relies on render pass

use std::{any::TypeId, collections::HashMap, sync::Arc};

use lazy_static::lazy_static;
use wgpu::{naga::Type, RenderPipeline};

use crate::{pipelines::{default_pipeline::DefaultPipeline, skybox_pipeline::SkyboxPipeline, ui_pipeline::UIPipeline}, render_context::RenderContext};

pub struct MyPipeline{
    pub pipeline: RenderPipeline,
    pub render_pass_builder: TypeId,
}

pub trait PipelineBuilder{
    fn build_pipeline(&self, render_context: &RenderContext) -> MyPipeline;
}

lazy_static!{
    pub static ref PIPELINE_BUILDERS: Arc<HashMap<TypeId, Box<dyn PipelineBuilder + Send + Sync>>> ={
        Arc::new(HashMap::from([
            (TypeId::of::<DefaultPipeline>(), Box::new(DefaultPipeline) as Box<dyn PipelineBuilder + Send + Sync>),
            (TypeId::of::<SkyboxPipeline>(), Box::new(SkyboxPipeline) as Box<dyn PipelineBuilder + Send + Sync>),
            (TypeId::of::<UIPipeline>(), Box::new(UIPipeline) as Box<dyn PipelineBuilder + Send + Sync>),
        ])
    )};
}