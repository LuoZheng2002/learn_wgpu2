// use std::sync::Arc;

// use wgpu::{RenderPipeline, util::DeviceExt};

// use crate::{
//     pipelines::default_pipeline::DefaultPipeline, render_context::RenderContext, render_data::RenderData, render_pipeline::PIPELINE_CACHE, renderable::Renderable, texture::Texture, vertex::Vertex
// };

// pub struct Polygon;

// impl Renderable for Polygon {
//     fn choose_pipeline(&self, render_context: &RenderContext) -> Arc<RenderPipeline> {
//         let mut pipeline_cache = PIPELINE_CACHE.lock().unwrap();
//         pipeline_cache.get_pipeline::<DefaultPipeline>(render_context) // 2.
//         // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
//         // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // 3.
//         // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
//         // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
//     }

//     fn load_data(&self, render_context: &RenderContext) -> RenderData {
//         const VERTICES: &[Vertex] = &[
//             Vertex {
//                 position: [-0.0868241, 0.49240386, 0.0],
//                 tex_coords: [0.4131759, 0.99240386],
//             }, // A
//             Vertex {
//                 position: [-0.49513406, 0.06958647, 0.0],
//                 tex_coords: [0.0048659444, 0.56958647],
//             }, // B
//             Vertex {
//                 position: [-0.21918549, -0.44939706, 0.0],
//                 tex_coords: [0.28081453, 0.05060294],
//             }, // C
//             Vertex {
//                 position: [0.35966998, -0.3473291, 0.0],
//                 tex_coords: [0.85967, 0.1526709],
//             }, // D
//             Vertex {
//                 position: [0.44147372, 0.2347359, 0.0],
//                 tex_coords: [0.9414737, 0.7347359],
//             }, // E
//         ];

//         const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
//         let device = &render_context.device;
//         let queue = &render_context.queue;
//         let camera_buffer = &render_context.camera_buffer;
//         let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Vertex Buffer"),
//             contents: bytemuck::cast_slice(VERTICES),
//             usage: wgpu::BufferUsages::VERTEX,
//         });

//         let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Index Buffer"),
//             contents: bytemuck::cast_slice(INDICES),
//             usage: wgpu::BufferUsages::INDEX,
//         });
//         let num_indices = INDICES.len() as u32;
//         let texture = Texture::from_bytes(
//             device,
//             queue,
//             include_bytes!("../../assets/grass.jpg"),
//             "texture",
//         )
//         .unwrap();
//         let bind_groups = DefaultPipeline::create_bind_groups(device, texture, camera_buffer);
//         RenderData {
//             vertex_buffer,
//             index_buffer,
//             bind_groups,
//             num_indices,
//         }
//     }
//     // functions to load the data, but where to store them?
// }
