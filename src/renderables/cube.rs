use std::sync::Arc;

use wgpu::{RenderPipeline, util::DeviceExt};

use crate::{
    pipelines::default_pipeline::DefaultPipeline, render_context::RenderContext, render_data::RenderData, render_pipeline::PIPELINE_CACHE, renderable::Renderable, texture::Texture, vertex::Vertex
};

pub struct Cube;

impl Renderable for Cube {
    fn choose_pipeline(&self, render_context: &RenderContext) -> Arc<RenderPipeline> {
        let mut pipeline_cache = PIPELINE_CACHE.lock().unwrap();
        pipeline_cache.get_pipeline::<DefaultPipeline>(render_context) // 2.
        // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
        // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // 3.
        // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    fn load_data(&self, render_context: &RenderContext) -> RenderData {
        #[rustfmt::skip]
        const VERTICES: &[Vertex] = &[
            // Front face (Z = 1)
            Vertex {position: [-1.0, -1.0, 1.0],tex_coords: [1./4., 2./3.],},
            Vertex {position: [1.0, -1.0, 1.0],tex_coords: [2./4., 2./3.],},
            Vertex {position: [1.0, 1.0, 1.0],tex_coords: [2./4., 1./3.],},
            Vertex {position: [-1.0, 1.0, 1.0],tex_coords: [1./4., 1./3.],},
            // Back face (Z = -1)
            Vertex {position: [-1.0, -1.0, -1.0],tex_coords: [3./4., 2./3.],},
            Vertex {position: [1.0, -1.0, -1.0],tex_coords: [1.0, 2./3.],},
            Vertex {position: [1.0, 1.0, -1.0],tex_coords: [1.0, 1./3.],},
            Vertex {position: [-1.0, 1.0, -1.0],tex_coords: [3./4., 1./3.],},
            // Left face (X = -1)
            Vertex {position: [-1.0, -1.0, -1.0],tex_coords: [0.0, 2./3.],},
            Vertex {position: [-1.0, -1.0, 1.0],tex_coords: [1./4., 2./3.],},
            Vertex {position: [-1.0, 1.0, 1.0],tex_coords: [1./4., 1./3.],},
            Vertex {position: [-1.0, 1.0, -1.0],tex_coords: [0.0, 1./3.],},
            // Right face (X = 1)
            Vertex {position: [1.0, -1.0, -1.0],tex_coords: [0.0, 0.0],},
            Vertex {position: [1.0, 1.0, -1.0],tex_coords: [0.0, 1.0],},
            Vertex {position: [1.0, 1.0, 1.0],tex_coords: [1.0, 1.0],},
            Vertex {position: [1.0, -1.0, 1.0],tex_coords: [1.0, 0.0],},
            // Top face (Y = 1)
            Vertex {position: [-1.0, 1.0, -1.0],tex_coords: [0.0, 0.0],},
            Vertex {position: [-1.0, 1.0, 1.0],tex_coords: [0.0, 1.0],},
            Vertex {position: [1.0, 1.0, 1.0],tex_coords: [1.0, 1.0],},
            Vertex {position: [1.0, 1.0, -1.0],tex_coords: [1.0, 0.0],},
            // Bottom face (Y = -1)
            Vertex {position: [-1.0, -1.0, -1.0],tex_coords: [0.0, 0.0],},
            Vertex {position: [1.0, -1.0, -1.0],tex_coords: [1.0, 0.0],},
            Vertex {position: [1.0, -1.0, 1.0],tex_coords: [1.0, 1.0],},
            Vertex {position: [-1.0, -1.0, 1.0],tex_coords: [0.0, 1.0],},
        ];

        const INDICES: &[u16] = &[
            0, 1, 2, 2, 3, 0, // Front
            4, 6, 5, 6, 4, 7, // Back
            8, 9, 10, 10, 11, 8, // Left
            12, 13, 14, 14, 15, 12, // Right
            16, 17, 18, 18, 19, 16, // Top
            20, 21, 22, 22, 23, 20, // Bottom
        ];
        let device = &render_context.device;
        let queue = &render_context.queue;
        let camera_buffer = &render_context.camera_buffer;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;
        let texture = Texture::from_bytes(
            device,
            queue,
            include_bytes!("../../assets/grass.jpg"),
            "texture",
        )
        .unwrap();
        let bind_groups = DefaultPipeline::create_bind_groups(device, texture, camera_buffer);
        RenderData {
            vertex_buffer,
            index_buffer,
            bind_groups,
            num_indices,
        }
    }
    // functions to load the data, but where to store them?
}
