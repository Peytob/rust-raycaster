use glm::{uvec2, Vec2};

use crate::game::graphics::RenderingState;
use crate::game::model::tilemap::{PlacedTile, Tilemap};

pub struct HitDetails {
    column: u32,
    total_columns: u32,
    ray: Ray,
    hit: Hit
}

impl HitDetails {
    pub fn new(column: u32, total_columns: u32, ray: Ray, hit: Hit) -> Self {
        Self { column, total_columns, ray, hit }
    }

    pub fn column(&self) -> u32 {
        self.column
    }
    pub fn total_columns(&self) -> u32 {
        self.total_columns
    }
    pub fn ray(&self) -> &Ray {
        &self.ray
    }
    pub fn hit(&self) -> &Hit {
        &self.hit
    }
}

pub enum Hit {
    None,
    Wall {
        placed_tile: PlacedTile,
    }
}

pub struct Ray {
    start_position: Vec2,
    end_position: Vec2,
    direction_angle: f32,
    distance: f32,
    maximal_distance: f32
}

impl Ray {
    pub fn new(start_position: Vec2, end_position: Vec2, direction_angle: f32, maximal_distance: f32) -> Self {
        Self { start_position, end_position, direction_angle, distance: 0.0, maximal_distance }
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
    pub fn maximal_distance(&self) -> f32 {
        self.maximal_distance
    }
}

// TODO Return custom iterator with computing rays on .next() call instead of std::vec
pub fn cast_rays(tilemap: &Tilemap, rendering_state: &RenderingState) -> Vec<HitDetails> {
    let camera_direction = rendering_state.camera.direction();
    let camera_position = rendering_state.camera.position();
    let total_columns = rendering_state.total_columns();

    let mut hits_buffer = Vec::<HitDetails>::with_capacity(total_columns as usize);

    for column in 0..total_columns {
        let ray_angle = camera_direction + relative_ray_angle(column, total_columns);
        let (ray, hit) = cast_ray(tilemap, camera_position, ray_angle, rendering_state.rendering_distance());
        hits_buffer.push(HitDetails::new(column, total_columns, ray, hit));
    }

    return hits_buffer;
}

pub fn cast_ray(tilemap: &Tilemap, start_position: Vec2, ray_angle: f32, maximal_distance: f32) -> (Ray, Hit) {
    // Todo Use binary algorithm with dynamic step size

    const STEP_SIZE: f32 = 0.05;

    let mut ray = Ray::new(start_position, start_position.clone(), ray_angle, maximal_distance);

    while ray.distance < maximal_distance {
        ray.end_position.x += STEP_SIZE * ray_angle.cos();
        ray.end_position.y += STEP_SIZE * ray_angle.sin();

        let current_tile = uvec2(ray.end_position.x as u32, ray.end_position.y as u32);

        if current_tile.x >= tilemap.sizes().x || current_tile.y >= tilemap.sizes().y {
            break;
        }

        let placed_tile = tilemap.get_tile(current_tile);

        if placed_tile.is_some_and(|tile| tile.tile().is_collision_enabled() ) {
            let hit = Hit::Wall {
                placed_tile: placed_tile.unwrap().clone(),
            };

            return (ray, hit)
        }

        ray.distance += STEP_SIZE;
    }

    return (ray, Hit::None);
}

pub fn relative_ray_angle(column: u32, total_columns: u32) -> f32 {
    ((column as f32) / (total_columns as f32)) - 0.5f32
}
