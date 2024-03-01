use glm::{UVec2, uvec2};

use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

#[derive(Copy, Clone)]
pub struct PlacedTile {
    tile_id: ResourceId
}

impl PlacedTile {
    pub fn new(tile_id: ResourceId) -> Self {
        Self { tile_id }
    }


    pub fn tile_id(&self) -> ResourceId {
        self.tile_id
    }
}

pub struct Tilemap {
    id: ResourceId,
    tiles: Vec<Vec<PlacedTile>>,
    sizes: UVec2
}

impl Tilemap {
    pub fn new(id: ResourceId, sizes: UVec2, empty_filler_tile_id: ResourceId) -> Self {
        let tiles = vec![vec![PlacedTile::new(empty_filler_tile_id); sizes.x as usize]; sizes.y as usize];
        Self {
            id,
            tiles,
            sizes
        }
    }

    pub fn from_raw_tilemap(id: ResourceId, raw_tilemap: Vec<Vec<ResourceId>>) -> Self {
        let sizes = uvec2(raw_tilemap.len() as u32, raw_tilemap.get(0).unwrap().len() as u32);

        let tiles = raw_tilemap.iter()
            .map(|row| row.iter()
                .map(|resource_id| PlacedTile::new(resource_id.clone()))
                .collect()
            )
            .collect();

        Self {
            id,
            tiles,
            sizes
        }
    }

    pub fn get_tile(&self, position: UVec2) -> Option<&PlacedTile> {
        self.tiles
            .get(position.y as usize)
            .map(|row| row.get(position.x as usize))
            .flatten()
    }

    pub fn set_tile(&mut self, position: UVec2, tile_id: ResourceId) {
        self.tiles
            .get_mut(position.y as usize)
            .map(|row| row[position.x as usize] = PlacedTile::new(tile_id));
    }
}

impl Resource for Tilemap {
    fn id(&self) -> ResourceId {
        self.id
    }
}
