use std::cell::RefCell;
use std::rc::Rc;

use glm::{uvec2, UVec2, Vec2};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::game::graphics::model::camera::Camera;
use crate::game::graphics::ray_caster::{cast_rays, Hit, Ray};
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

const TC: u32 = 90;

impl Renderer {
    pub fn new(canvas: &Rc<RefCell<WindowCanvas>>, tile_repository: &Rc<RefCell<Repository<Tile>>>) -> Self {
        Self { canvas: canvas.clone(), tile_repository: tile_repository.clone() }
    }

    pub fn render_tilemap(&self, tilemap: &Tilemap, rendering_state: &RenderingState, camera: &Camera) {
        let total_columns = TC;

        for (hit, hit_details) in cast_rays(tilemap, camera, rendering_state.rendering_distance, total_columns) {
            match hit {
                Hit::None { ray } => {
                    self.draw_column(&ray, hit_details.column(), hit_details.total_columns(), &Color::WHITE);
                }

                Hit::Wall { placed_tile, ray } => {
                    let color = self.tile_repository.borrow().get_resource(&placed_tile.tile_id()).unwrap().color().clone();
                    self.draw_column(&ray, hit_details.column(), hit_details.total_columns(), &color);
                }
            };
        }
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

            self.draw_2d_line(&camera_position, &camera_direction_second_point, &Color::RED);
        }

        // Rendering throwing camera rays
        {
            let total_columns = TC;

            for (hit, _) in cast_rays(tilemap, camera, rendering_state.rendering_distance, total_columns) {
                match hit {
                    Hit::None { ray } => {
                        self.draw_2d_line(&ray.start_position(), &ray.end_position(), &Color::BLACK)
                    }

                    Hit::Wall { placed_tile, ray } => {
                        self.draw_2d_line(&ray.start_position(), &ray.end_position(), &Color::BLACK)
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

        let color = self.tile_repository.borrow().get_resource(&placed_tile.tile_id().clone()).unwrap().color();

        canvas.set_draw_color(color);

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

    fn draw_2d_line(&self, from: &Vec2, to: &Vec2, color: &Color) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(*color);

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

    fn draw_column(&self, ray: &Ray, column: u32, total_column: u32, color: &Color) {
        let mut canvas = self.canvas.borrow_mut();

        let (width, height) = canvas.window().size();

        let column_width = (width as f32 * (1f32 / total_column as f32)) as u32 + 1;

        let wall_height = 1.0f32 / ray.distance();
        let view_wall_height = ((height as f32) * wall_height.min(1.0f32)) as u32;

        let wall_top = (height - view_wall_height) / 2;
        let wall_bottom = wall_top + view_wall_height;

        let column_x = (column_width * column) as i32;

        // Ceiling
        canvas.set_draw_color(Color::BLACK);
        let ceiling_rect = Rect::new(
            column_x,
            0,
            column_width,
            wall_top
        );
        canvas.fill_rect(ceiling_rect).unwrap();

        // Wall
        canvas.set_draw_color(compute_shaded_color(color, ray));
        let column_rect = Rect::new(
            column_x,
            wall_top as i32,
            column_width,
            wall_bottom - wall_top
        );
        canvas.fill_rect(column_rect).unwrap();

        // Floor
        canvas.set_draw_color(Color::GRAY);
        let ceiling_rect = Rect::new(
            column_x,
            wall_bottom as i32,
            column_width,
            height - wall_bottom
        );
        canvas.fill_rect(ceiling_rect).unwrap();
    }
}

fn compute_shaded_color(color: &Color, ray: &Ray) -> Color {
    let covered_distance = ray.distance() / ray.maximal_distance();
    let shadow_activation_border = 0.25f32;

    if covered_distance < shadow_activation_border {
        return color.clone()
    }

    let shadow_power = shadow_activation_border / covered_distance;

    return Color::from((
        (color.r as f32 * shadow_power) as u8,
        (color.g as f32 * shadow_power) as u8,
        (color.b as f32 * shadow_power) as u8
    ));
}
