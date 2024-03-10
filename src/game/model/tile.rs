use sdl2::pixels::Color;
use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

pub struct Tile {
    id: ResourceId,
    color: Color
}

impl Tile {
    pub fn new(id: ResourceId, color: Color) -> Self {
        Self { id, color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Resource for Tile {
    fn id(&self) -> ResourceId {
        self.id
    }
}
