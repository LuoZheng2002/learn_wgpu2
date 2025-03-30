use std::collections::{BTreeMap, BTreeSet, HashMap};


use crate::{render_context::RenderContext, renderable::Renderable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RenderPassType{
    Opaque3D,
    UI
}

pub struct RenderPassCreator<'a>{
    color_view: Option<&'a wgpu::TextureView>,
    depth_view: Option<&'a wgpu::TextureView>,
}
impl<'a> RenderPassCreator<'a>{
    fn create_render_pass_from_type(&self, encoder: &'a mut wgpu::CommandEncoder, render_pass_type: RenderPassType) -> wgpu::RenderPass::<'a>{
        match render_pass_type {
            RenderPassType::Opaque3D => {
                create_render_pass(encoder, self.color_view, self.depth_view, true)
            }
            RenderPassType::UI => {
                create_render_pass(encoder, self.color_view, None, false)
            }
        }
    }
}

pub struct RenderPasses<'a>{
    renderable_refs: BTreeMap<RenderPassType, Vec<&'a mut dyn Renderable>>,
    render_pass_creator: Option<RenderPassCreator<'a>>,
    render_context: &'a RenderContext,
    pipeline_cache: &'a mut PipelineCache,
}

impl <'a> RenderPasses<'a>{
    pub fn new(renderables: &'a mut Vec<Box<dyn Renderable + Send + Sync + 'static>>,
        render_context: &'a mut RenderContext,
        pipeline_cache: &'a mut PipelineCache,
        color_view: Option<&'a wgpu::TextureView>
    ) -> Self{
        let mut renderable_refs: BTreeMap<RenderPassType, Vec<&'a mut dyn Renderable>> = BTreeMap::new();
        for renderable in renderables.iter_mut().map(|renderable|renderable){
            let render_pass_type = renderable.get_render_pass_type(render_context, pipeline_cache);
            let renderable_ref = renderable.as_mut();            
            renderable_refs.entry(render_pass_type).or_insert(vec![]).push(renderable_ref);
        }
        let depth_view = Some(&render_context.depth_texture.view);
        Self{
            renderable_refs,
            render_pass_creator: Some(RenderPassCreator{
                color_view,
                depth_view,
            }),
            render_context,
            pipeline_cache,
        }
    }
    pub fn render(&mut self, encoder: &'a mut wgpu::CommandEncoder){
        let render_pass_types: BTreeSet<RenderPassType> = self.renderable_refs.keys().cloned().collect();
        let render_pass_creator = self.render_pass_creator.take().unwrap();
        for render_pass_type in render_pass_types {            
            let mut render_pass = render_pass_creator.create_render_pass_from_type(encoder, render_pass_type);
            let renderables = self.renderable_refs.get_mut(&render_pass_type).unwrap();
            for renderable in renderables {
                renderable.render(&mut render_pass, self.render_context, self.pipeline_cache);
            }
        }
    }
    
}   



pub fn create_render_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, color_view: Option<&'a wgpu::TextureView>,
    depth_view: Option<&'a wgpu::TextureView>,
    clear_color: bool)-> wgpu::RenderPass<'a>{
    let mut color_attachments: Vec<Option<wgpu::RenderPassColorAttachment>> = vec![];
    if let Some(view) = color_view {
        color_attachments.push(Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: if clear_color{wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                })}else{wgpu::LoadOp::Load},
                store: wgpu::StoreOp::Store,
            },
        }));
    }
    let depth_stencil_attachment = depth_view.map(|view| wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    });
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &color_attachments,
        depth_stencil_attachment,
        occlusion_query_set: None,
        timestamp_writes: None,
    })
}