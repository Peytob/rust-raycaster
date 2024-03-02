use std::cell::RefCell;
use std::rc::Rc;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use num_traits::Zero;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use crate::game::ecs::component::direction_component::DirectionComponent;
use crate::game::ecs::component::player_flag_component::PlayerFlagComponent;
use crate::game::ecs::component::position_component::PositionComponent;

const PLAYER_MOVING_SPEED: f32 = 0.5f32; // Tiles
const PLAYER_ROTATION_SPEED: f32 = 0.05f32;

pub struct MovingSystem {
    event_pump: Rc<RefCell<EventPump>>
}

// Can be decomposed

impl MovingSystem {

    pub fn new(event_pump: &Rc<RefCell<EventPump>>) -> Self {
        Self {
            event_pump: event_pump.clone()
        }
    }

    pub fn move_player(&self, entity_id: &usize, manager: &mut EntityManager) {
        match manager.borrow_component_pair_mut::<PositionComponent, DirectionComponent>(*entity_id) {
            None => {}
            Some((position_component, direction_component)) => {
                self.handle_player_moving(position_component, direction_component);
            }
        }
    }
    fn handle_player_moving(&self, position_component: &mut PositionComponent, direction_component: &mut DirectionComponent) {
        let mut moving_difference = 0.0f32;
        let mut rotation_difference = 0.0f32;

        for scancode in self.event_pump.borrow_mut().keyboard_state().pressed_scancodes() {
            match scancode {
                Scancode::A => { rotation_difference += PLAYER_ROTATION_SPEED }
                Scancode::D => { rotation_difference -= PLAYER_ROTATION_SPEED }
                Scancode::W => { moving_difference += PLAYER_MOVING_SPEED }
                Scancode::S => { moving_difference -= PLAYER_MOVING_SPEED }

                _ => {}
            }
        }

        if !rotation_difference.is_zero() {
            direction_component.direction = direction_component.direction + rotation_difference;
        }

        if !moving_difference.is_zero() {
            position_component.position = position_component.position + direction_component.direction * moving_difference;
        }

        // info!("Dif: rotation {}; moving {}", rotation_difference, moving_difference);
        // info!("Direction {} {}", direction_component.direction.x, direction_component.direction.y);
        // info!("Position {} {}", position_component.position.x, position_component.position.y);
    }
}

impl System for MovingSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let player_id = accessor
            .borrow_ids_for_triple::<PositionComponent, DirectionComponent, PlayerFlagComponent>(manager)
            .map(|player_entities_ids| player_entities_ids.get(0))
            .flatten();

        match player_id {
            None => {}
            Some(player_id) => self.move_player(player_id, manager)
        }
    }
}
