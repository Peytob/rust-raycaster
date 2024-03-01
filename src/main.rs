mod game;

use env_logger::Env;
use crate::game::Game;

fn main() {
    initialize_logging();
    log::info!("Application started");

    log::info!("Initializing SDL context");
    let sdl_context = sdl2::init().unwrap();
    log::info!("SDL context has been initialized");

    let mut game = Game::initialize_game(sdl_context);
    game.run_game_loop()
}

fn initialize_logging() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);
}
