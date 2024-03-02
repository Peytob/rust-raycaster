use glm::Vec2;

pub struct Camera {
    position: Vec2,
    direction: f32,
    fov: f32
}

impl Camera {

    pub fn new(position: Vec2, direction: f32, fov: f32) -> Self {
        Self { position, direction, fov }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn direction(&self) -> f32 {
        self.direction
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
    pub fn set_direction(&mut self, direction: f32) {
        self.direction = direction;
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
}
