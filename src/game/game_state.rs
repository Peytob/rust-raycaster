pub struct GameState {
    is_game_running: bool
}

impl GameState {
    pub fn new() -> Self {
        Self {
            is_game_running: true
        }
    }
    pub fn is_game_running(&self) -> bool {
        self.is_game_running
    }

    pub fn set_is_game_running(&mut self, is_game_running: bool) {
        self.is_game_running = is_game_running;
    }
}
