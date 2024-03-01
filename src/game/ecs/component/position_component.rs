use ecs_rust::component::Component;
use glm::Vec2;

pub struct PositionComponent {
    pub position: Vec2
}

impl PositionComponent {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Component for PositionComponent {
}
