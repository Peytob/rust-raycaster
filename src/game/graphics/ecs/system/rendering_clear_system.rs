use std::cell::RefCell;
use std::rc::Rc;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use crate::game::graphics::renderer::Renderer;

pub struct RenderingClearSystem {
    renderer: Rc<RefCell<Renderer>>
}

impl RenderingClearSystem {
    pub fn new(renderer: &Rc<RefCell<Renderer>>) -> Self {
        Self {
            renderer: renderer.clone()
        }
    }
}

impl System for RenderingClearSystem {
    fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        self.renderer.borrow_mut().clear();
    }
}
