use ecs_rust::component::Component;
use crate::game::graphics::model::camera::Camera;

pub struct CameraComponent {
    camera: Camera
}

impl CameraComponent {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

impl Component for CameraComponent {
}
