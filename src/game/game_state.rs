use std::cell::RefCell;
use std::rc::Rc;
use crate::game::model::linemap::Linemap;
use crate::game::model::repository::Repository;
use crate::game::model::tile::Tile;
use crate::game::model::tilemap::Tilemap;

pub struct GameState {
    is_game_running: bool,
    repositories: Repositories
}

pub struct Repositories {
    tiles_repository: Rc<RefCell<Repository<Tile>>>,
    tilemap_repository: Rc<RefCell<Repository<Tilemap>>>,
    linemap_repository: Rc<RefCell<Repository<Linemap>>>
}

impl Repositories {
    pub fn new() -> Self {
        Self {
            tiles_repository: Rc::new(RefCell::new(Repository::new())),
            tilemap_repository: Rc::new(RefCell::new(Repository::new())),
            linemap_repository: Rc::new(RefCell::new(Repository::new()))
        }
    }

    pub fn tiles_repository(&self) -> &Rc<RefCell<Repository<Tile>>> {
        &self.tiles_repository
    }
    pub fn tilemap_repository(&self) -> &Rc<RefCell<Repository<Tilemap>>> {
        &self.tilemap_repository
    }
    pub fn linemap_repository(&self) -> &Rc<RefCell<Repository<Linemap>>> {
        &self.linemap_repository
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            is_game_running: true,
            repositories: Repositories::new()
        }
    }

    pub fn is_game_running(&self) -> bool {
        self.is_game_running
    }
    pub fn set_is_game_running(&mut self, is_game_running: bool) {
        self.is_game_running = is_game_running;
    }
    pub fn repositories(&self) -> &Repositories {
        &self.repositories
    }
}
