use ecs_rust::component::Component;
use glm::Vec2;

pub struct DirectionComponent {
    pub direction: Vec2
}

impl DirectionComponent {
    pub fn new(direction: Vec2) -> Self {
        Self { direction }
    }
}

impl Component for DirectionComponent {
}
