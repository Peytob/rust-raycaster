use std::time::Duration;
use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::game::graphic::Graphics;

mod graphic;

#[must_use]
pub struct Game {
    running_flag: bool,
    graphics: Graphics,
    event_pump: EventPump,
}

trait GameModule {
}

impl Game {
    pub fn initialize_game(sdl_context: Sdl) -> Game {
        log::info!("Initializing game");

        let graphics = Graphics::initialize_graphics(&sdl_context);
        let mut event_pump = sdl_context.event_pump().unwrap();

        log::info!("Game has been initialized");

        Game {
            running_flag: true,
            graphics,
            event_pump
        }
    }

    pub fn run_game_loop(mut self) {
        log::info!("Starting game cycle");

        'running: loop {
            self.handle_window_events();

            if (self.running_flag == false) {
                break 'running
            }
        }

        log::info!("Exiting from game cycle");
    }

    fn handle_window_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.running_flag = false
                },

                _ => {}
            }
        }
    }
}
