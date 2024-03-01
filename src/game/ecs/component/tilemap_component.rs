use ecs_rust::component::Component;
use crate::game::model::ResourceId;

pub struct TilemapComponent {
    tilemap_id: ResourceId
}

impl TilemapComponent {
    pub fn new(tilemap: ResourceId) -> Self {
        Self { tilemap_id: tilemap }
    }

    pub fn tilemap(&self) -> &ResourceId {
        &self.tilemap_id
    }
}

impl Component for TilemapComponent {
}
