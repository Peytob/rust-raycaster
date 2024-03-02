use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use crate::game::ecs::component::direction_component::DirectionComponent;
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
        let camera_ids = accessor.borrow_ids_for_triple::<CameraComponent, PositionComponent, DirectionComponent>(manager).unwrap();

        for camera_entity_id in camera_ids {
            let (mut camera_component, position_component, direction_component) = manager
                .borrow_component_triple_mut::<CameraComponent, PositionComponent, DirectionComponent>(*camera_entity_id).unwrap();

            let camera = camera_component.camera_mut();

            camera.set_position(position_component.position.clone());
            camera.set_direction(direction_component.direction);
        }
    }
}
