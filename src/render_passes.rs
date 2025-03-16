use std::collections::{BTreeMap, BTreeSet, HashMap};


use crate::{render_context::RenderContext, renderable::Renderable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RenderPassType{
    Opaque3D,
    UI
}

pub struct RenderPasses<'a>{
    renderable_refs: BTreeMap<RenderPassType, Vec<&'a mut dyn Renderable>>,
}

impl <'a> RenderPasses<'a>{
    pub fn new(renderables: &'a mut Vec<Box<dyn Renderable>>, render_context: &mut RenderContext) -> Self{
        let mut renderable_refs: BTreeMap<RenderPassType, Vec<&'a mut dyn Renderable>> = BTreeMap::new();
        for renderable in renderables.iter_mut().map(|renderable|renderable){
            let render_pass_type = renderable.get_render_pass_type(render_context);
            let renderable_ref = renderable.as_mut();            
            renderable_refs.entry(render_pass_type).or_insert(vec![]).push(renderable_ref);
        }
        Self{
            renderable_refs,
        }
    }
    pub fn render(&mut self, encoder: &mut wgpu::CommandEncoder, render_context: &mut RenderContext){
        let render_pass_types: BTreeSet<RenderPassType> = self.renderable_refs.keys().cloned().collect();
        for render_pass_type in render_pass_types {
            let renderables = self.renderable_refs.get_mut(&render_pass_type).unwrap();
            let mut render_pass = create_render_pass_from_type(encoder, render_pass_type);
            for renderable in renderables {
                renderable.render(&mut render_pass, render_context);
            }
        }
    }
}

pub fn create_render_pass_from_type(encoder: &mut wgpu::CommandEncoder, render_pass_type: RenderPassType) -> wgpu::RenderPass{
    match render_pass_type {
        RenderPassType::Opaque3D => {
            create_render_pass(encoder, None, None)
        }
        RenderPassType::UI => {
            create_render_pass(encoder, None, None)
        }
    }
}

pub fn create_render_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, color_view: Option<wgpu::TextureView>,
    depth_view: Option<wgpu::TextureView>)-> wgpu::RenderPass<'a>{
    let mut color_attachments: Vec<Option<wgpu::RenderPassColorAttachment>> = vec![];
    if let Some(view) = color_view.as_ref() {
        color_attachments.push(Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        }));
    }
    let depth_stencil_attachment = depth_view.as_ref().map(|view| wgpu::RenderPassDepthStencilAttachment {
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