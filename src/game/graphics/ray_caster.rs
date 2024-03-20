use glm::{uvec2, Vec2};
use sdl2::pixels::Color;

use crate::game::graphics::RenderingState;
use crate::game::model::linemap::{Line, Linemap};
use crate::game::model::object_color::ObjectColor;
use crate::game::model::tilemap::Tilemap;

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
        color: Color,
        is_collision_enabled: bool
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
pub fn cast_rays_tilemap(tilemap: &Tilemap, rendering_state: &RenderingState) -> Vec<HitDetails> {
    let camera_direction = rendering_state.camera.direction();
    let camera_position = rendering_state.camera.position();
    let total_columns = rendering_state.total_columns();

    let mut hits_buffer = Vec::<HitDetails>::with_capacity(total_columns as usize);

    for column in 0..total_columns {
        let ray_angle = camera_direction + relative_ray_angle(column, total_columns);
        let (ray, hit) = cast_ray_tilemap(tilemap, camera_position, ray_angle, rendering_state.rendering_distance());
        hits_buffer.push(HitDetails::new(column, total_columns, ray, hit));
    }

    return hits_buffer;
}

pub fn cast_ray_tilemap(tilemap: &Tilemap, start_position: Vec2, ray_angle: f32, maximal_distance: f32) -> (Ray, Hit) {
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
            let placed_tile = placed_tile.unwrap();
            let hit = Hit::Wall {
                color: Color::BLACK,// placed_tile.tile().color().clone(),
                is_collision_enabled: placed_tile.tile().is_collision_enabled()
            };

            return (ray, hit)
        }

        ray.distance += STEP_SIZE;
    }

    return (ray, Hit::None);
}

pub fn cast_rays_linemap(linemap: &Linemap, rendering_state: &RenderingState) -> Vec<HitDetails> {
    let camera_direction = rendering_state.camera.direction();
    let camera_position = rendering_state.camera.position();
    let total_columns = rendering_state.total_columns();

    let mut hits_buffer = Vec::<HitDetails>::with_capacity(total_columns as usize);

    for column in 0..total_columns {
        let ray_angle = camera_direction + relative_ray_angle(column, total_columns);
        let (ray, hit) = cast_ray_linemap(linemap, camera_position, ray_angle, rendering_state.rendering_distance());
        hits_buffer.push(HitDetails::new(column, total_columns, ray, hit));
    }

    return hits_buffer;
}

pub fn cast_ray_linemap(linemap: &Linemap, start_position: Vec2, ray_angle: f32, maximal_distance: f32) -> (Ray, Hit) {
    let mut ray = Ray::new(start_position, start_position.clone(), ray_angle, maximal_distance);
    ray.distance = maximal_distance;

    ray.end_position.x += maximal_distance * ray_angle.cos();
    ray.end_position.y += maximal_distance * ray_angle.sin();

    let ray_line = Line::new(ObjectColor::WHITE, ray.start_position.clone(), ray.end_position.clone());
    let mut hit = Hit::None;

    for line in linemap.lines() {
        match Line::find_intersection(line, &ray_line) {
            None => {}
            Some(intersection) => {
                let intersection_distance = glm::distance(ray.start_position, intersection);
                if intersection_distance < ray.distance {
                    ray.end_position = intersection;
                    ray.distance = intersection_distance;
                    hit = Hit::Wall {
                        color: Color::BLACK, // line.color().clone(),
                        is_collision_enabled: true
                    }
                }
            }
        };
    }

    return (ray, hit)
}

pub fn relative_ray_angle(column: u32, total_columns: u32) -> f32 {
    ((column as f32) / (total_columns as f32)) - 0.5f32
}
