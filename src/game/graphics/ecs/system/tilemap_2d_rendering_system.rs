use std::cell::RefCell;
use std::rc::Rc;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use crate::game::ecs::component::tilemap_component::TilemapComponent;
use crate::game::graphics::renderer::Renderer;
use crate::game::graphics::RenderingState;
use crate::game::graphics::tilemap_2d_render::{render_camera_2d, render_tilemap_2d};
use crate::game::model::repository::Repository;
use crate::game::model::tilemap::Tilemap;

pub struct Tilemap2DRenderingSystem {
    renderer: Rc<RefCell<Renderer>>,
    rendering_state: Rc<RefCell<RenderingState>>,
    tilemap_repository: Rc<RefCell<Repository<Tilemap>>>
}

impl Tilemap2DRenderingSystem {
    pub fn new(renderer: &Rc<RefCell<Renderer>>, rendering_state: &Rc<RefCell<RenderingState>>, tilemap_repository: &Rc<RefCell<Repository<Tilemap>>>) -> Self {
        Self {
            renderer: renderer.clone(),
            rendering_state: rendering_state.clone(),
            tilemap_repository: tilemap_repository.clone()
        }
    }
}

impl System for Tilemap2DRenderingSystem {

    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {

        let tilemap_repository = self.tilemap_repository.borrow();
        let rendering_state = self.rendering_state.borrow();
        let renderer = self.renderer.borrow();

        for tilemap_entity_id in accessor.borrow_ids::<TilemapComponent>(manager).unwrap() {
            let tilemap_id = manager
                .borrow_component::<TilemapComponent>(*tilemap_entity_id)
                .unwrap()
                .tilemap();

            let tilemap = tilemap_repository.get_resource(tilemap_id).unwrap();
            render_tilemap_2d(tilemap, &rendering_state, &renderer);
            render_camera_2d(tilemap, &rendering_state, &renderer);
        }
    }
}
