pub mod model;
pub mod ecs;
mod renderer;

use std::cell::RefCell;
use std::rc::Rc;
use sdl2::Sdl;
use crate::game::graphics::renderer::Renderer;

#[must_use]
pub struct Graphics {
    renderer: Rc<RefCell<Renderer>>,
    rendering_state: Rc<RefCell<RenderingState>>
}

impl Graphics {

    pub fn initialize_graphics(sdl_context: &Sdl) -> Graphics {
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
    rendering_distance: f32
}

impl RenderingState {
    pub fn new() -> Rc<RefCell<Self>> {
        let rendering_state = Self {
            rendering_distance: 5.0
        };

        Rc::new(RefCell::new(rendering_state))
    }
}
