use sdl2::render::WindowCanvas;
use sdl2::Sdl;

#[must_use]
pub struct Graphics {
    canvas: WindowCanvas,
    rendering_state: RenderingState
}

impl Graphics {

    pub fn initialize_graphics(sdl_context: &Sdl) -> Graphics {
        log::info!("Initializing graphics module");

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Raymarcher", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        log::info!("Initializing graphics module has been initialized");

        return Graphics {
            rendering_state: RenderingState::new(),
            canvas
        }
    }


    pub fn canvas(&self) -> &WindowCanvas {
        &self.canvas
    }
    pub fn rendering_state(&self) -> &RenderingState {
        &self.rendering_state
    }
}

pub struct RenderingState {
    fov: f32
}

impl RenderingState {
    pub fn new() -> Self {
        Self {
            fov: 100.0
        }
    }
}
