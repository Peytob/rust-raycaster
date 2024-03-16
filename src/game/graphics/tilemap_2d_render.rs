use glm::{uvec2, Vec2};
use sdl2::pixels::Color;
use crate::game::graphics::ray_caster::{cast_rays, Hit};
use crate::game::graphics::renderer::Renderer;
use crate::game::graphics::RenderingState;
use crate::game::model::tilemap::Tilemap;

pub fn render_tilemap_2d(tilemap: &Tilemap, rendering_state: &RenderingState, renderer: &Renderer) {
    for x in 0..tilemap.sizes().x {
        for y in 0..tilemap.sizes().y {
            let tile_position = uvec2(x, y);
            let tile = tilemap.get_tile(tile_position).unwrap();
            renderer.render_2d_placed_tile(&tile_position, tile);
        }
    }
}

pub fn render_camera_2d(tilemap: &Tilemap, rendering_state: &RenderingState, renderer: &Renderer) {
    let camera = &rendering_state.camera;

    let camera_position = camera.position();
    renderer.render_2d_point(&camera_position, 10);

    let camera_direction = camera.direction();

    // Rendering camera direction red ray
    {
        const CAMERA_DIRECTION_RAY_LEN: f32 = 3.0;

        let camera_direction_second_point = Vec2::new(
            &camera_position.x + CAMERA_DIRECTION_RAY_LEN * camera_direction.cos(),
            &camera_position.y + CAMERA_DIRECTION_RAY_LEN * camera_direction.sin(),
        );

        renderer.render_2d_line(&camera_position, &camera_direction_second_point, &Color::RED);
    }

    // Rendering throwing camera rays
    {
        for hit_details in cast_rays(tilemap, &rendering_state) {
            let ray = hit_details.ray();
            match hit_details.hit() {
                Hit::None => {
                    renderer.render_2d_line(&ray.start_position(), &ray.end_position(), &Color::BLACK)
                }

                Hit::Wall { .. } => {
                    renderer.render_2d_line(&ray.start_position(), &ray.end_position(), &Color::BLACK)
                }
            };
        }
    }
}
