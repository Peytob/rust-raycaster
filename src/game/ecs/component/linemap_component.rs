use ecs_rust::component::Component;
use crate::game::model::ResourceId;

pub struct LinemapComponent {
    linemap: ResourceId
}

impl LinemapComponent {
    pub fn new(linemap: ResourceId) -> Self {
        Self { linemap }
    }

    pub fn linemap(&self) -> ResourceId {
        self.linemap
    }
}

impl Component for LinemapComponent {
}
