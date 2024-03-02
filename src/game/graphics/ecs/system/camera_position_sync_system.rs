use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use crate::game::ecs::component::position_component::PositionComponent;
use crate::game::graphics::ecs::component::camera_component::CameraComponent;

pub struct CameraPositionSyncSystem;

impl CameraPositionSyncSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for CameraPositionSyncSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let camera_ids = accessor.borrow_ids_for_pair::<CameraComponent, PositionComponent>(manager).unwrap();

        for camera_entity_id in camera_ids {
            let (mut camera_component, position_component) = manager
                .borrow_component_pair_mut::<CameraComponent, PositionComponent>(*camera_entity_id).unwrap();

            camera_component.camera_mut().set_position(position_component.position.clone());
        }
    }
}
