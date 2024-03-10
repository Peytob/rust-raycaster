use sdl2::pixels::Color;
use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

#[derive(Copy, Clone)]
pub struct Tile {
    id: ResourceId,
    color: Color,
    is_collision_enabled: bool
}

impl Tile {
    pub fn new(id: ResourceId, color: Color, is_collision_enabled: bool) -> Self {
        Self { id, color, is_collision_enabled }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn is_collision_enabled(&self) -> bool {
        self.is_collision_enabled
    }
}

impl Resource for Tile {
    fn id(&self) -> ResourceId {
        self.id
    }
}
