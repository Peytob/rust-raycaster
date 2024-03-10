use std::cell::RefCell;
use std::rc::Rc;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use crate::game::ecs::component::direction_component::DirectionComponent;
use crate::game::ecs::component::player_flag_component::PlayerFlagComponent;
use crate::game::ecs::component::position_component::PositionComponent;
use crate::game::graphics::RenderingState;

pub struct CameraPositionSyncSystem {
    rendering_state: Rc<RefCell<RenderingState>>,
}

impl CameraPositionSyncSystem {
    pub fn new(rendering_state: &Rc<RefCell<RenderingState>>) -> Self {
        Self {
            rendering_state: rendering_state.clone()
        }
    }
}

impl System for CameraPositionSyncSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let player_id = accessor.borrow_ids_for_triple::<PositionComponent, DirectionComponent, PlayerFlagComponent>(manager)
            .unwrap()
            .first()
            .unwrap();

        let position_component = manager
            .borrow_component::<PositionComponent>(*player_id).unwrap();

        let direction_component = manager
            .borrow_component::<DirectionComponent>(*player_id).unwrap();

        let mut rendering_state = self.rendering_state.borrow_mut();

        let camera = rendering_state.camera_mut();

        camera.set_position(position_component.position.clone());
        camera.set_direction(direction_component.direction);
    }
}
