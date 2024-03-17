pub mod model;
pub mod ecs;
mod renderer;
mod ray_caster;
mod tilemap_2d_render;
mod tilemap_3d_render;
mod linemap_2d_render;
mod linemap_3d_render;

use std::cell::RefCell;
use std::rc::Rc;
use glm::vec2;
use sdl2::Sdl;
use crate::game::game_state::Repositories;
use crate::game::graphics::model::camera::Camera;
use crate::game::graphics::renderer::Renderer;

#[must_use]
pub struct Graphics {
    renderer: Rc<RefCell<Renderer>>,
    rendering_state: Rc<RefCell<RenderingState>>
}

impl Graphics {

    pub fn initialize_graphics(sdl_context: &Sdl, repositories: &Repositories) -> Graphics {
        log::info!("Initializing graphics module");

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Raymarcher", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let canvas_ref = Rc::new(RefCell::new(canvas));

        log::info!("Initializing graphics module has been initialized");

        return Graphics {
            rendering_state: RenderingState::new(),
            renderer: Rc::new(RefCell::new(Renderer::new(&canvas_ref)))
        }
    }


    pub fn rendering_state(&self) -> &Rc<RefCell<RenderingState>> {
        &self.rendering_state
    }

    pub fn renderer(&self) -> &Rc<RefCell<Renderer>> {
        &self.renderer
    }
}

pub struct RenderingState {

    // Maximal rendering distance
    rendering_distance: f32,

    camera: Camera,

    // Total columns in 3d graphics
    total_columns: u32,
}

impl RenderingState {
    pub fn new() -> Rc<RefCell<Self>> {
        let rendering_state = Self {
            rendering_distance: 10.0,
            total_columns: 120,
            camera: Camera::new(vec2(3.0, 3.0), 0.0f32, 90f32.to_radians())
        };

        Rc::new(RefCell::new(rendering_state))
    }

    pub fn rendering_distance(&self) -> f32 {
        self.rendering_distance
    }
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn total_columns(&self) -> u32 {
        self.total_columns
    }
}
