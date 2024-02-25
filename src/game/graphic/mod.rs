use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::video::Window;
use crate::game::GameModule;

#[must_use]
pub struct Graphics {
    canvas: WindowCanvas
}

impl Graphics {

    pub fn initialize_graphics(sdl_context: &Sdl) -> (Graphics) {
        log::info!("Initializing graphics module");

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        log::info!("Initializing graphics module has been initialized");

        return Graphics { canvas }
    }
}

impl GameModule for Graphics {
}
