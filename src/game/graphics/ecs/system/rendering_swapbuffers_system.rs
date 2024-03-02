use std::cell::RefCell;
use std::rc::Rc;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use crate::game::graphics::renderer::Renderer;

pub struct RenderingSwapBuffersSystem {
    renderer: Rc<RefCell<Renderer>>,
}

impl RenderingSwapBuffersSystem {
    pub fn new(renderer: &Rc<RefCell<Renderer>>) -> Self {
        Self {
            renderer: renderer.clone()
        }
    }
}

impl System for RenderingSwapBuffersSystem {

    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        self.renderer.borrow_mut().show();
    }
}
