use std::cell::RefCell;
use std::rc::Rc;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

#[must_use]
pub struct Graphics {
    canvas: Rc<RefCell<WindowCanvas>>,
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

        log::info!("Initializing graphics module has been initialized");

        return Graphics {
            rendering_state: RenderingState::new(),
            canvas: Rc::new(RefCell::new(canvas))
        }
    }

    pub fn canvas(&self) -> &Rc<RefCell<WindowCanvas>> {
        &self.canvas
    }
    pub fn rendering_state(&self) -> &Rc<RefCell<RenderingState>> {
        &self.rendering_state
    }
}

pub struct RenderingState {
    fov: f32
}

impl RenderingState {
    pub fn new() -> Rc<RefCell<Self>> {
        let rendering_state = Self {
            fov: 100.0
        };

        Rc::new(RefCell::new(rendering_state))
    }
}
