use std::cell::RefCell;
use std::rc::Rc;

use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use glm::{uvec2, Vec2};
use crate::game::ecs::component::direction_component::DirectionComponent;

use crate::game::ecs::component::player_flag_component::PlayerFlagComponent;
use crate::game::ecs::component::position_component::PositionComponent;
use crate::game::ecs::component::tilemap_component::TilemapComponent;
use crate::game::model::is_exists_resource;
use crate::game::model::repository::Repository;
use crate::game::model::tilemap::Tilemap;

pub struct CollisionResolvingSystem {
    tilemap_repository: Rc<RefCell<Repository<Tilemap>>>
}

impl CollisionResolvingSystem {

    pub fn new(tilemap_repository: &Rc<RefCell<Repository<Tilemap>>>) -> Self {
        Self { tilemap_repository: tilemap_repository.clone() }
    }

    pub fn resolve_collision(&self, tilemap: &Tilemap, player_position: &mut Vec2, player_direction: &f32) {
        let mut current_tile = uvec2(player_position.x as u32, player_position.y as u32);
        let collision_resolution_step = 0.01f32;

        while tilemap.get_tile(current_tile).is_some_and(|placed_tile| is_exists_resource(placed_tile.tile_id())) {
            player_position.x -= player_direction.cos() * collision_resolution_step;
            player_position.y -= player_direction.sin() * collision_resolution_step;
            current_tile = uvec2(player_position.x as u32, player_position.y as u32);
        }
    }
}

// Can be decomposed

impl CollisionResolvingSystem {

}

impl System for CollisionResolvingSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let tilemap_id = accessor
            .borrow_ids::<TilemapComponent>(manager)
            .map(|tilemap_ids| tilemap_ids.get(0))
            .flatten()
            .map(|tilemap_id| manager.borrow_component::<TilemapComponent>(*tilemap_id))
            .flatten()
            .unwrap()
            .tilemap()
            .clone();

        let (player_position, player_direction) = accessor
            .borrow_ids_for_triple::<PositionComponent, DirectionComponent, PlayerFlagComponent>(manager)
            .map(|player_entities_ids| player_entities_ids.get(0))
            .flatten()
            .map(|player_id| manager.borrow_component_pair_mut::<PositionComponent, DirectionComponent>(*player_id))
            .flatten()
            .unwrap();

        match self.tilemap_repository.borrow_mut().get_resource(&tilemap_id) {
            None => {}
            Some(tilemap) => self.resolve_collision(&tilemap, &mut player_position.position, &player_direction.direction)
        };
    }
}
