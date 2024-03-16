use std::cell::RefCell;
use std::rc::Rc;

use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;

use crate::game::ecs::component::linemap_component::LinemapComponent;
use crate::game::graphics::linemap_2d_render::render_linemap_2d;
use crate::game::graphics::renderer::Renderer;
use crate::game::graphics::RenderingState;
use crate::game::model::linemap::Linemap;
use crate::game::model::repository::Repository;

pub struct Linemap2DRenderingSystem {
    renderer: Rc<RefCell<Renderer>>,
    rendering_state: Rc<RefCell<RenderingState>>,
    linemap_repository: Rc<RefCell<Repository<Linemap>>>
}

impl Linemap2DRenderingSystem {
    pub fn new(renderer: &Rc<RefCell<Renderer>>, rendering_state: &Rc<RefCell<RenderingState>>, linemap_repository: &Rc<RefCell<Repository<Linemap>>>) -> Self {
        Self {
            renderer: renderer.clone(),
            rendering_state: rendering_state.clone(),
            linemap_repository: linemap_repository.clone()
        }
    }
}

impl System for Linemap2DRenderingSystem {

    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let linemap_repository = self.linemap_repository.borrow();
        let rendering_state = self.rendering_state.borrow();
        let renderer = self.renderer.borrow();

        for linemap_entity_id in accessor.borrow_ids::<LinemapComponent>(manager).unwrap() {
            let linemap_id = manager
                .borrow_component::<LinemapComponent>(*linemap_entity_id)
                .unwrap()
                .linemap();

            let linemap = linemap_repository.get_resource(&linemap_id).unwrap();
            render_linemap_2d(linemap, &rendering_state, &renderer);
            // render_camera_2d(linemap, &rendering_state, &renderer);
        }
    }
}
