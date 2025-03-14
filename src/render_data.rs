use std::{any::TypeId, collections::HashMap, sync::Mutex};

use crate::{
    get_type::GetType,
    render_context::{self, RenderContext},
    renderable::Renderable,
};
use lazy_static::lazy_static;

pub struct RenderData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub bind_groups: Vec<wgpu::BindGroup>,
    pub num_indices: u32,
}

#[derive(Default)]
pub struct RenderDataCache {
    data: HashMap<TypeId, RenderData>,
}

impl RenderDataCache {
    pub fn get_data<T: Renderable + GetType + ?Sized>(
        &mut self,
        renderable: &T,
        render_context: &RenderContext,
    ) -> &RenderData {
        let type_id = renderable.get_type();
        self.data.entry(type_id).or_insert_with(|| {
            println!("Creating render data");
            renderable.load_data(render_context)
        })
    }
}

lazy_static! {
    pub static ref RENDER_DATA_CACHE: Mutex<RenderDataCache> =
        Mutex::new(RenderDataCache::default());
}
