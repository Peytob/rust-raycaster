use std::cell::RefCell;
use std::rc::Rc;
use glm::Vec3;
use sdl2::render::WindowCanvas;
use crate::game::graphics::model::camera::Camera;
use crate::game::graphics::RenderingState;
use crate::game::model::tilemap::Tilemap;

enum Hit {
    None,

    Wall {
        color: Vec3,
        distance: f32
    }
}

pub struct Renderer {
    canvas: Rc<RefCell<WindowCanvas>>
}

impl Renderer {
    pub fn new(canvas: &Rc<RefCell<WindowCanvas>>) -> Self {
        Self { canvas: canvas.clone() }
    }

    pub fn render_tilemap(&self, tilemap: &Tilemap, rendering_state: &RenderingState, camera: &Camera) {
    }

    pub fn render_tilemap_2d(&self, tilemap: &Tilemap, rendering_state: &RenderingState, camera: &Camera) {
    }

    pub fn show(&self) {
        self.canvas.borrow_mut().present();
    }

    fn cast_ray(&self, tilemap: &Tilemap, camera: &Camera) {

    }
}
