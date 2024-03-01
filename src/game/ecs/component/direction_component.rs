use ecs_rust::component::Component;

pub struct DirectionComponent {
    pub direction: f32 // Angle
}

impl DirectionComponent {
    pub fn new(direction: f32) -> Self {
        Self { direction }
    }
}

impl Component for DirectionComponent {
}
