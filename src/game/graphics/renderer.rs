use std::cell::RefCell;
use std::rc::Rc;

use glm::{uvec2, UVec2, Vec2};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::game::graphics::model::camera::Camera;
use crate::game::graphics::ray_caster::{cast_rays, Hit};
use crate::game::graphics::RenderingState;
use crate::game::model::is_exists_resource;
use crate::game::model::repository::Repository;
use crate::game::model::tile::Tile;
use crate::game::model::tilemap::{PlacedTile, Tilemap};

const TILE_SIZE: Vec2 = Vec2 { x: 32f32, y: 32f32 };

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

        for x in 0..tilemap.sizes().x {
            for y in 0..tilemap.sizes().y {
                let tile_position = uvec2(x, y);
                let tile = tilemap.get_tile(tile_position).unwrap();
                self.render_placed_tile_2d(&tile_position, tile);
            }
        }

        self.render_camera_2d(tilemap, camera, rendering_state)
    }

    pub fn render_camera_2d(&self, tilemap: &Tilemap, camera: &Camera, rendering_state: &RenderingState) {
        let camera_position = camera.position();
        self.render_point_2d(&camera_position, 10);

        let camera_direction = camera.direction();

        // Rendering camera direction red ray
        {
            let camera_direction_ray_len = 3.0;

            let camera_direction_second_point = Vec2::new(
                &camera_position.x + camera_direction_ray_len * camera_direction.cos(),
                &camera_position.y + camera_direction_ray_len * camera_direction.sin(),
            );

            self.draw_2d_line(&camera_position, &camera_direction_second_point, Color::RED);
        }

        // Rendering throwing camera rays
        {
            // Todo CastRays method

            let total_columns = 30;

            for hit in cast_rays(tilemap, camera, rendering_state.rendering_distance, total_columns) {
                match hit {
                    Hit::None { ray } => {
                        self.draw_2d_line(&ray.start_position(), &ray.end_position(), Color::BLACK)
                    }
                    Hit::Wall { placed_tile, ray } => {
                        self.draw_2d_line(&ray.start_position(), &ray.end_position(), Color::BLACK)
                    }
                };
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

    fn render_placed_tile_2d(&self, tile_position: &UVec2, placed_tile: &PlacedTile) {
        if !is_exists_resource(placed_tile.tile_id()) {
            return;
        }

        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(Color::GREEN);
        let tile_rect = Rect::new(
            (tile_position.x as f32 * TILE_SIZE.x) as i32,
            (tile_position.y as f32 * TILE_SIZE.y) as i32,
            TILE_SIZE.x as u32,
            TILE_SIZE.y as u32
        );

        canvas.fill_rect(tile_rect).unwrap();
    }

    fn render_point_2d(&self, point_center: &Vec2, point_size: u32) {
        let mut canvas = self.canvas.borrow_mut();

        let camera_point_rect = Rect::new(
            (point_center.x * TILE_SIZE.x) as i32 - (point_size / 2) as i32,
            (point_center.y * TILE_SIZE.y) as i32 - (point_size / 2) as i32,
            point_size,
            point_size
        );

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(camera_point_rect).unwrap()
    }

    fn draw_2d_line(&self, from: &Vec2, to: &Vec2, color: Color) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(color);

        canvas.draw_line(
            Point::new(
                (from.x * TILE_SIZE.x) as i32,
                (from.y * TILE_SIZE.y) as i32
            ),

            Point::new(
                (to.x * TILE_SIZE.x) as i32,
                (to.y * TILE_SIZE.y) as i32
            )
        ).unwrap()
    }
}
