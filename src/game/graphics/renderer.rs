use std::cell::RefCell;
use std::rc::Rc;
use glm::{uvec2, UVec2, vec2, Vec2, Vec3, vec3};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::game::graphics::model::camera::Camera;
use crate::game::graphics::RenderingState;
use crate::game::model::is_exists_resource;
use crate::game::model::repository::Repository;
use crate::game::model::tile::Tile;
use crate::game::model::tilemap::{PlacedTile, Tilemap};

enum Hit {
    None,

    Wall {
        color: Vec3,
        distance: f32
    }
}

pub struct Renderer {
    canvas: Rc<RefCell<WindowCanvas>>,
    tile_repository: Rc<RefCell<Repository<Tile>>>
}

impl Renderer {
    pub fn new(canvas: &Rc<RefCell<WindowCanvas>>, tile_repository: &Rc<RefCell<Repository<Tile>>>) -> Self {
        Self { canvas: canvas.clone(), tile_repository: tile_repository.clone() }
    }

    pub fn render_tilemap(&self, tilemap: &Tilemap, rendering_state: &RenderingState, camera: &Camera) {
    }

    pub fn render_tilemap_2d(&self, tilemap: &Tilemap, rendering_state: &RenderingState, camera: &Camera) {
        // Todo send sizes
        let (window_width, window_height) = self.canvas.borrow().window().size();
        let tile_size = uvec2(window_width, window_height) / tilemap.sizes();

        for x in 0..tilemap.sizes().x {
            for y in 0..tilemap.sizes().y {
                let tile_position = uvec2(x, y);
                let tile = tilemap.get_tile(tile_position).unwrap();
                self.render_placed_tile_2d(&tile_position, &tile_size, tile);
            }
        }

        let camera_position = camera.position();
        self.render_camera_position_2d(&camera_position, &tile_size);

        let camera_direction = camera.direction();

        // Rendering camera direction red ray
        {
            let camera_direction_ray_len = 3.0;

            let camera_direction_second_point = Vec2::new(
                &camera_position.x + camera_direction_ray_len * camera_direction.cos(),
                &camera_position.y + camera_direction_ray_len * camera_direction.sin(),
            );

            self.draw_2d_line(&camera_position, &camera_direction_second_point, &tile_size, Color::RED);
        }

        // Rendering throwing camera rays
        {
            let fov = camera.fov();
            let total_columns = 30;

            for column in 0..total_columns {
                let ray_angle = camera_direction + self.relative_ray_angle(column, total_columns);

                let distance = match self.cast_ray(tilemap, camera_position, ray_angle, rendering_state.rendering_distance) {
                    Hit::None => { rendering_state.rendering_distance }
                    Hit::Wall { color: _color, distance } => { distance }
                };

                let to = vec2(
                    camera_position.x + distance * ray_angle.cos(),
                    camera_position.y + distance * ray_angle.sin()
                );

                self.draw_2d_line(&camera_position, &to, &tile_size, Color::BLACK)
            }
        }
    }

    pub fn clear(&self) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
    }

    pub fn show(&self) {
        self.canvas.borrow_mut().present();
    }

    fn cast_ray(&self, tilemap: &Tilemap, start_position: Vec2, ray_angle: f32, maximal_distance: f32) -> Hit {
        // Todo Use binary algorithm with dynamic step size

        let mut distance = 0.0f32;
        let step_size = 0.05;
        let mut current_position = start_position.clone();

        while distance < maximal_distance {
            current_position.x += step_size * ray_angle.cos();
            current_position.y += step_size * ray_angle.sin();

            let current_tile = uvec2(current_position.x as u32, current_position.y as u32);

            if current_tile.x < 0 || current_tile.x >= tilemap.sizes().x || current_tile.y < 0 || current_tile.y >= tilemap.sizes().y {
                return Hit::Wall { color: vec3(1f32, 0f32, 0f32), distance }
            }

            if tilemap.get_tile(current_tile).is_some_and(|tile| is_exists_resource(tile.tile_id()) ) {
                return Hit::Wall { color: vec3(1f32, 0f32, 0f32), distance }
            }

            distance += step_size;
        }

        Hit::None
    }

    fn render_placed_tile_2d(&self, tile_position: &UVec2, tile_size: &UVec2, placed_tile: &PlacedTile) {
        if !is_exists_resource(placed_tile.tile_id()) {
            return;
        }

        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(Color::GREEN);
        let tile_rect = Rect::new(
            (tile_position.x * tile_size.x) as i32,
            (tile_position.y * tile_size.y) as i32,
            tile_size.x,
            tile_size.y
        );

        canvas.fill_rect(tile_rect).unwrap();
    }

    fn render_camera_position_2d(&self, camera_position: &Vec2, tile_size: &UVec2) {
        let mut canvas = self.canvas.borrow_mut();

        let camera_quad_size = 10u32;

        let camera_point_rect = Rect::new(
            (camera_position.x * tile_size.x as f32) as i32 - (camera_quad_size / 2) as i32,
            (camera_position.y * tile_size.y as f32) as i32 - (camera_quad_size / 2) as i32,
            camera_quad_size,
            camera_quad_size
        );

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(camera_point_rect).unwrap()
    }

    fn draw_2d_line(&self, from: &Vec2, to: &Vec2, tile_size: &UVec2, color: Color) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(color);

        canvas.draw_line(
            Point::new(
                (from.x * tile_size.x as f32) as i32,
                (from.y * tile_size.y as f32) as i32
            ),

            Point::new(
                (to.x * tile_size.x as f32) as i32,
                (to.y * tile_size.y as f32) as i32
            )
        ).unwrap()
    }
    fn relative_ray_angle(&self, column: i32, total_columns: i32) -> f32 {
        ((column as f32) / (total_columns as f32)) - 0.5f32
    }
}
