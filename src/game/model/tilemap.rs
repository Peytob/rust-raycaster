use std::rc::Rc;

use glm::{UVec2, uvec2};

use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;
use crate::game::model::tile::Tile;

pub struct PlacedTile {
    tile: Rc<Tile>
}

impl Clone for PlacedTile {
    fn clone(&self) -> Self {
        PlacedTile::new(&self.tile())
    }
}

impl PlacedTile {
    pub fn new(tile: &Rc<Tile>) -> Self {
        Self { tile: tile.clone() }
    }


    pub fn tile(&self) -> &Rc<Tile> {
        &self.tile
    }
}

pub struct Tilemap {
    id: ResourceId,
    tiles: Vec<Vec<PlacedTile>>,
    sizes: UVec2
}

impl Tilemap {
    pub fn new(id: ResourceId, sizes: UVec2, empty_tile: &Rc<Tile>) -> Self {
        let tiles = vec![vec![PlacedTile::new(empty_tile); sizes.x as usize]; sizes.y as usize];
        Self {
            id,
            tiles,
            sizes
        }
    }

    pub fn from_raw_tilemap(id: ResourceId, raw_tilemap: Vec<Vec<&Rc<Tile>>>) -> Self {
        let sizes = uvec2(raw_tilemap.len() as u32, raw_tilemap.get(0).unwrap().len() as u32);

        let tiles = raw_tilemap.iter()
            .map(|row| row.iter()
                .map(|tile| PlacedTile::new(tile))
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

    pub fn set_tile(&mut self, position: UVec2, tile: &Rc<Tile>) {
        self.tiles
            .get_mut(position.y as usize)
            .map(|row| row[position.x as usize] = PlacedTile::new(tile));
    }

    pub fn sizes(&self) -> UVec2 {
        self.sizes
    }
}

impl Resource for Tilemap {
    fn id(&self) -> ResourceId {
        self.id
    }
}
