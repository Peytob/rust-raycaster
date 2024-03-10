use std::cell::RefCell;
use std::rc::Rc;

use glm::{UVec2, Vec2};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::game::graphics::ray_caster::Ray;
use crate::game::model::is_exists_resource;
use crate::game::model::repository::Repository;
use crate::game::model::tile::Tile;
use crate::game::model::tilemap::PlacedTile;

const TILE_SIZE: Vec2 = Vec2 { x: 32f32, y: 32f32 };

pub struct Renderer {
    canvas: Rc<RefCell<WindowCanvas>>,
    tile_repository: Rc<RefCell<Repository<Tile>>>
}

impl Renderer {
    pub fn new(canvas: &Rc<RefCell<WindowCanvas>>, tile_repository: &Rc<RefCell<Repository<Tile>>>) -> Self {
        Self { canvas: canvas.clone(), tile_repository: tile_repository.clone() }
    }

    pub fn clear(&self) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
    }

    pub fn show(&self) {
        self.canvas.borrow_mut().present();
    }

    pub fn render_2d_placed_tile(&self, tile_position: &UVec2, placed_tile: &PlacedTile) {
        let tile_repository = self.tile_repository.borrow();
        let mut canvas = self.canvas.borrow_mut();

        let color = if is_exists_resource(placed_tile.tile_id()) {
            tile_repository.get_resource(&placed_tile.tile_id().clone()).unwrap().color()
        } else {
            Color::WHITE
        };

        canvas.set_draw_color(color);

        let tile_rect = Rect::new(
            (tile_position.x as f32 * TILE_SIZE.x) as i32,
            (tile_position.y as f32 * TILE_SIZE.y) as i32,
            TILE_SIZE.x as u32,
            TILE_SIZE.y as u32
        );

        canvas.fill_rect(tile_rect).unwrap();
    }

    pub fn render_2d_point(&self, point_center: &Vec2, point_size: u32) {
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

    pub fn render_2d_line(&self, from: &Vec2, to: &Vec2, color: &Color) {
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

    pub fn render_column(&self, ray: &Ray, column: u32, total_column: u32, color: &Color) {
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
    const SHADOW_ACTIVATION_BORDER: f32 = 0.25f32;

    if covered_distance < SHADOW_ACTIVATION_BORDER {
        return color.clone()
    }

    let shadow_power = SHADOW_ACTIVATION_BORDER / covered_distance;

    return Color::from((
        (color.r as f32 * shadow_power) as u8,
        (color.g as f32 * shadow_power) as u8,
        (color.b as f32 * shadow_power) as u8
    ));
}
