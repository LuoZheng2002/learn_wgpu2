use std::any::{Any, TypeId};
pub trait GetType {
    fn get_type(&self) -> TypeId;
}

impl<T: 'static> GetType for T {
    fn get_type(&self) -> TypeId {
        TypeId::of::<T>()
    }
}