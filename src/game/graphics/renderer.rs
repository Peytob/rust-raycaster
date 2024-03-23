use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use glm::{UVec2, Vec2};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::game::graphics::ray_caster::{Hit, HitDetails, Ray};
use crate::game::graphics::RenderingState;
use crate::game::model::object_color::ObjectColor;
use crate::game::model::tilemap::PlacedTile;

const TILE_SIZE: Vec2 = Vec2 { x: 32f32, y: 32f32 };

pub struct Renderer {
    canvas: Rc<RefCell<WindowCanvas>>
}

impl Renderer {
    pub fn new(canvas: &Rc<RefCell<WindowCanvas>>) -> Self {
        Self { canvas: canvas.clone() }
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
        let mut canvas = self.canvas.borrow_mut();

        let color = placed_tile.tile().color();

        let tile_rect = Rect::new(
            (tile_position.x as f32 * TILE_SIZE.x) as i32,
            (tile_position.y as f32 * TILE_SIZE.y) as i32,
            TILE_SIZE.x as u32,
            TILE_SIZE.y as u32
        );

        draw_colored_rect(&mut canvas, &tile_rect, color)
    }

    pub fn render_2d_rect(&self, color: &ObjectColor, position: &Vec2, size: &Vec2) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(*resolve_object_color(color));

        let tile_rect = Rect::new(
            position.x as i32,
            position.y as i32,
            size.x as u32,
            size.y as u32
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

    pub fn render_2d_line(&self, from: &Vec2, to: &Vec2, color: &ObjectColor) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(*resolve_object_color(color));

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

    fn render_column(&self, ray: &Ray, column: u32, total_column: u32, color: &ObjectColor) {
        let mut canvas = self.canvas.borrow_mut();

        let (width, height) = canvas.window().size();

        let column_width = (width as f32 * (1f32 / total_column as f32)) as u32 + 1;

        let wall_height = 1.0f32 / ray.distance();
        let view_wall_height = ((height as f32) * wall_height.min(1.0f32)) as u32;

        let wall_top = (height - view_wall_height) / 2;
        let wall_bottom = wall_top + view_wall_height;

        let column_x = (column_width * column) as i32;

        // Ceiling
        const SKY_COLOR: ObjectColor = ObjectColor::COLOR { color: &Color::RGB(135, 206, 235) };
        let ceiling_rect = Rect::new(
            column_x,
            0,
            column_width,
            wall_top
        );
        draw_colored_rect(&mut canvas, &ceiling_rect, &SKY_COLOR);

        // Wall
        let column_rect = Rect::new(
            column_x,
            wall_top as i32,
            column_width,
            wall_bottom - wall_top
        );
        draw_colored_rect(&mut canvas, &column_rect, &color);

        // Floor
        let ceiling_rect = Rect::new(
            column_x,
            wall_bottom as i32,
            column_width,
            height - wall_bottom
        );
        draw_colored_rect(&mut canvas, &ceiling_rect, &ObjectColor::GRAY);
    }
}

fn draw_colored_rect(canvas: &mut WindowCanvas, rect: &Rect, color: &ObjectColor) {
    match color {
        ObjectColor::COLOR { color } => {
            canvas.set_draw_color(**color);
            canvas.fill_rect(*rect).unwrap();
        }
        ObjectColor::TEXTURE { .. } => {}
    }
}

fn resolve_object_color(color: &ObjectColor) -> &'static Color {
    return match color {
        ObjectColor::COLOR { color } => {
            color
        }

        ObjectColor::TEXTURE { .. } => {
            &Color::BLACK
        }
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

pub fn render_hit_line(hit_details: &HitDetails, _rendering_state: &RenderingState, renderer: &Renderer) {
    let ray = hit_details.ray();
    match hit_details.hit() {
        Hit::None => {
            renderer.render_2d_line(&ray.start_position(), &ray.end_position(), &ObjectColor::BLACK)
        }

        Hit::Wall { .. } => {
            renderer.render_2d_line(&ray.start_position(), &ray.end_position(), &ObjectColor::BLACK)
        }
    };
}

pub fn render_hit_column(hit_details: &HitDetails, _rendering_state: &RenderingState, renderer: &Renderer) {
    let ray = hit_details.ray();
    match hit_details.hit() {
        Hit::None => {
            renderer.render_column(&ray, hit_details.column(), hit_details.total_columns(), &ObjectColor::WHITE);
        }

        Hit::Wall { color, .. } => {
            renderer.render_column(&ray, hit_details.column(), hit_details.total_columns(), &color);
        }
    };
}
