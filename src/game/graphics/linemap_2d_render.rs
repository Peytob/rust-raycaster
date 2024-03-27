use glm::Vec2;
use num_traits::zero;

use crate::game::graphics::ray_caster::cast_rays_linemap;
use crate::game::graphics::renderer::{render_hit_line, Renderer};
use crate::game::graphics::RenderingState;
use crate::game::model::linemap::Linemap;
use crate::game::model::object_color::ObjectColor;

pub fn render_linemap_2d(linemap: &Linemap, _rendering_state: &RenderingState, renderer: &Renderer) {
    renderer.render_2d_rect(&ObjectColor::WHITE, &zero(), &linemap.sizes());

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

        renderer.render_2d_line(&camera_position, &camera_direction_second_point, &ObjectColor::RED);
    }

    for hit_details in cast_rays_linemap(linemap, &rendering_state) {
        render_hit_line(&hit_details, rendering_state, renderer);
    }
}
