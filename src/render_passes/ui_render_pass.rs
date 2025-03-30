use crate::my_render_pass::RenderPassBuilder;

pub struct UiRenderPass;

impl RenderPassBuilder for UiRenderPass{
    fn create_render_pass<'a>(
        &self,
        encoder: &'a mut wgpu::CommandEncoder,
        color_view: &'a wgpu::TextureView,
        depth_view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        let _ = depth_view;
        let color_attachment = Some(wgpu::RenderPassColorAttachment {
                view: &color_view,
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
            });
        // In a UI render pass, we do not use a depth buffer
        // let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
        //     view: depth_view,
        //     depth_ops: Some(wgpu::Operations {
        //         load: wgpu::LoadOp::Clear(1.0),
        //         store: wgpu::StoreOp::Store,
        //     }),
        //     stencil_ops: None,
        // };
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[color_attachment],
            depth_stencil_attachment: None, // In a UI render pass, we do not use a depth buffer
            occlusion_query_set: None,
            timestamp_writes: None,
        };
        encoder.begin_render_pass(&render_pass_descriptor)
    }
}