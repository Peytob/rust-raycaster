mod graphics;
mod game_state;
mod ecs;

use ecs_rust::world::World;
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;
use crate::game::ecs::component::player_flag_component::PlayerFlagComponent;
use crate::game::ecs::component::position_component::PositionComponent;
use crate::game::ecs::system::moving_system::MovingSystem;
use crate::game::game_state::GameState;
use crate::game::graphics::Graphics;

pub struct Game {
    graphics: Graphics,
    event_pump: EventPump,
    world: World,

    game_state: GameState
}

impl Game {

    pub fn initialize_game(sdl_context: Sdl) -> Game {
        log::info!("Initializing game");

        let graphics = Graphics::initialize_graphics(&sdl_context);
        let mut event_pump = sdl_context.event_pump().unwrap();

        log::info!("Game has been initialized");

        log::info!("Initializing ECS world");

        let world = Game::create_ecs_world();

        log::info!("ECS world has been initialized");

        Game {
            graphics,
            event_pump,
            world,

            game_state: GameState::new()
        }
    }

    fn create_ecs_world() -> World {
        let mut world = World::new();

        // Registering components
        world
            .register_component::<PositionComponent>()
            .register_component::<PlayerFlagComponent>();

        // Creating systems
        world
            // Input and events handling systems
            .add_system(MovingSystem::new());

        world
    }

    pub fn run_game_loop(&mut self) {

        'main_game_loop: loop {
            self.handle_events();

            self.world.update();

            if !self.game_state.is_game_running() {
                break 'main_game_loop;
            }
        }
    }

    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.game_state.set_is_game_running(false)
                },

                _ => {}
            }
        }
    }
}
