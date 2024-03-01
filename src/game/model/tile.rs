use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

pub struct Tile {
    id: ResourceId
}

impl Tile {
    pub fn new(id: ResourceId) -> Self {
        Self { id }
    }
}

impl Resource for Tile {
    fn id(&self) -> ResourceId {
        self.id
    }
}
