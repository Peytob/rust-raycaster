use glm::Vec2;
use num_traits::zero;
use sdl2::pixels::Color;
use crate::game::graphics::ray_caster::{cast_rays_linemap, cast_rays_tilemap, Hit};

use crate::game::graphics::renderer::Renderer;
use crate::game::graphics::RenderingState;
use crate::game::model::linemap::Linemap;

pub fn render_linemap_2d(linemap: &Linemap, rendering_state: &RenderingState, renderer: &Renderer) {
    renderer.render_2d_rect(&Color::WHITE, &zero(), &linemap.sizes());

    for line in linemap.lines() {
        renderer.render_2d_line(line.from(), line.to(), line.color())
    }
}

pub fn render_camera_2d(linemap: &Linemap, rendering_state: &RenderingState, renderer: &Renderer) {
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
        for hit_details in cast_rays_linemap(linemap, &rendering_state) {
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
