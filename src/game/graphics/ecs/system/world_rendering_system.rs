use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use crate::game::ecs::component::tilemap_component::TilemapComponent;
use crate::game::graphics::ecs::component::camera_component::CameraComponent;
use crate::game::graphics::renderer::Renderer;
use crate::game::graphics::RenderingState;
use crate::game::model::repository::Repository;
use crate::game::model::tilemap::Tilemap;

pub struct WorldRenderingSystem {
    renderer: Rc<RefCell<Renderer>>,
    rendering_state: Rc<RefCell<RenderingState>>,
    tilemap_repository: Rc<RefCell<Repository<Tilemap>>>
}

impl WorldRenderingSystem {
    pub fn new(renderer: &Rc<RefCell<Renderer>>, rendering_state: &Rc<RefCell<RenderingState>>, tilemap_repository: &Rc<RefCell<Repository<Tilemap>>>) -> Self {
        Self {
            renderer: renderer.clone(),
            rendering_state: rendering_state.clone(),
            tilemap_repository: tilemap_repository.clone()
        }
    }
}

impl System for WorldRenderingSystem {

    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let camera_option = accessor.borrow_ids::<CameraComponent>(manager)
            .map(|cameras_ids| cameras_ids.first())
            .flatten()
            .map(|camera_entity| manager
                .borrow_component::<CameraComponent>(*camera_entity)
                .unwrap()
                .camera());

        match camera_option {
            None => {}
            Some(camera) => {
                let tilemap_repository = self.tilemap_repository.borrow();
                let renderer = self.renderer.borrow();

                for tilemap_entity_id in accessor.borrow_ids::<TilemapComponent>(manager).unwrap() {
                    let tilemap_id = manager
                        .borrow_component::<TilemapComponent>(*tilemap_entity_id)
                        .unwrap()
                        .tilemap();

                    let tilemap = tilemap_repository.get_resource(tilemap_id).unwrap();

                    renderer.render_tilemap(tilemap, self.rendering_state.borrow().deref(), camera)
                }
            }
        };
    }
}
