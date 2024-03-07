use glm::{uvec2, Vec2};

use crate::game::graphics::model::camera::Camera;
use crate::game::model::is_exists_resource;
use crate::game::model::tilemap::{PlacedTile, Tilemap};

pub enum Hit {
    None {
        ray: Ray
    },

    Wall {
        placed_tile: PlacedTile,
        ray: Ray
    }
}

pub struct Ray {
    start_position: Vec2,
    end_position: Vec2,
    direction_angle: f32,
    distance: f32
}

impl Ray {
    pub fn new(start_position: Vec2, end_position: Vec2, direction_angle: f32, distance: f32) -> Self {
        Self { start_position, end_position, direction_angle, distance }
    }

    pub fn start_position(&self) -> Vec2 {
        self.start_position
    }
    pub fn end_position(&self) -> Vec2 {
        self.end_position
    }
    pub fn direction_angle(&self) -> f32 {
        self.direction_angle
    }
    pub fn distance(&self) -> f32 {
        self.distance
    }
}

// TODO Return custom iterator with computing rays on .next() call instead of std::vec
pub fn cast_rays(tilemap: &Tilemap, camera: &Camera, maximal_distance: f32, total_columns: u32) -> Vec<Hit> {
    let camera_direction = camera.direction();
    let camera_position = camera.position();
    let mut hits_buffer = Vec::<Hit>::with_capacity(total_columns as usize);

    for column in 0..total_columns {
        let ray_angle = camera_direction + relative_ray_angle(column, total_columns);
        let hit = cast_ray(tilemap, camera_position, ray_angle, maximal_distance);
        hits_buffer.push(hit);
    }

    return hits_buffer;
}

pub fn cast_ray(tilemap: &Tilemap, start_position: Vec2, ray_angle: f32, maximal_distance: f32) -> Hit {
    // Todo Use binary algorithm with dynamic step size

    let step_size = 0.05;

    let mut ray = Ray::new(start_position, start_position.clone(), ray_angle, 0.0);

    while ray.distance < maximal_distance {
        ray.end_position.x += step_size * ray_angle.cos();
        ray.end_position.y += step_size * ray_angle.sin();

        let current_tile = uvec2(ray.end_position.x as u32, ray.end_position.y as u32);

        if current_tile.x < 0 || current_tile.x >= tilemap.sizes().x || current_tile.y < 0 || current_tile.y >= tilemap.sizes().y {
            return Hit::None { ray }
        }

        let placed_tile = tilemap.get_tile(current_tile);

        if placed_tile.is_some_and(|tile| is_exists_resource(tile.tile_id()) ) {
            return Hit::Wall {
                placed_tile: placed_tile.unwrap().clone(),
                ray,
            }
        }

        ray.distance += step_size;
    }

    Hit::None { ray }
}

pub fn relative_ray_angle(column: u32, total_columns: u32) -> f32 {
    ((column as f32) / (total_columns as f32)) - 0.5f32
}
