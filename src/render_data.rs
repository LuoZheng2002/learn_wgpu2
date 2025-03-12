use std::{any::TypeId, collections::HashMap, sync::{Arc, Mutex}};

use crate::renderable::Renderable;
use lazy_static::lazy_static;

pub struct RenderData{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub num_indices: u32,
}

#[derive(Default)]
pub struct RenderDataCache{
    data: HashMap<TypeId, Arc<RenderData>>,
}

impl RenderDataCache{
    pub fn get_data<T>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, bind_group_layout: &wgpu::BindGroupLayout)->Arc<RenderData>
    where
        T: Renderable + Default + 'static,
    {
        if let Some(data) = self.data.get(&TypeId::of::<T>()){
            data.clone()
        } else {
            let generator = T::default();
            let data = generator.load_data(device, queue, bind_group_layout);
            self.data.insert(TypeId::of::<T>(), Arc::new(data));
            self.data.get(&TypeId::of::<T>()).unwrap().clone()
        }
    }
}

lazy_static!{
    pub static ref RENDER_DATA_CACHE: Mutex<RenderDataCache> = Mutex::new(RenderDataCache::default());
}