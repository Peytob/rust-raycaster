use sdl2::pixels::Color;
use crate::game::model::object_color::ObjectColor;
use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

pub struct Tile {
    id: ResourceId,
    color: ObjectColor,
    is_collision_enabled: bool
}

impl Tile {
    pub fn new(id: ResourceId, color: ObjectColor, is_collision_enabled: bool) -> Self {
        Self {
            id,
            color,
            is_collision_enabled
        }
    }

    pub fn color(&self) -> &ObjectColor {
        &self.color
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
