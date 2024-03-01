use ecs_rust::component::Component;

pub struct PlayerFlagComponent;

impl PlayerFlagComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for PlayerFlagComponent {
}
