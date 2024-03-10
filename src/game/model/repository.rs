use std::collections::HashMap;
use std::rc::Rc;
use crate::game::model::ResourceId;

pub trait Resource {
    fn id(&self) -> ResourceId;
}

pub struct Repository<T : Resource> {
    data: HashMap<ResourceId, Rc<T>>
}

impl<T : Resource> Repository<T> {
    pub fn new() -> Self {
        Self { data: HashMap::with_capacity(32) }
    }

    pub fn get_resource(&self, id: &ResourceId) -> Option<&Rc<T>> {
        self.data.get(&id)
    }

    pub fn register_resource(&mut self, resource: Rc<T>) -> &mut Self {
        self.data.insert(resource.id(), resource);
        self
    }
}
