use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;

pub struct MovingSystem;

impl MovingSystem {

    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl System for MovingSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        
    }
}
