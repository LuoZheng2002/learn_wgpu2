use std::{any::TypeId, collections::HashMap, sync::Arc};

use lazy_static::lazy_static;

use crate::render_passes::{opauqe3d_render_pass::Opaque3DRenderPass, ui_render_pass::UiRenderPass};



pub trait RenderPassBuilder{
    /// This function should create a render pass based on the provided parameters.
    fn create_render_pass<'a>(
        &self,
        encoder: &'a mut wgpu::CommandEncoder,
        color_view: &'a wgpu::TextureView,
        depth_view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a>;
}

lazy_static!{
    pub static ref RENDER_PASS_BUILDERS: Vec<(TypeId, Box<dyn RenderPassBuilder + Send + Sync>)> ={
        Vec::from([
            (TypeId::of::<Opaque3DRenderPass>(), Box::new(Opaque3DRenderPass) as Box<dyn RenderPassBuilder + Send + Sync>),
            (TypeId::of::<UiRenderPass>(), Box::new(UiRenderPass) as Box<dyn RenderPassBuilder + Send + Sync>),
        ])
    };
}