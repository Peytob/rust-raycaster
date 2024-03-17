use crate::game::graphics::ray_caster::cast_rays_linemap;
use crate::game::graphics::renderer::{render_hit_column, Renderer};
use crate::game::graphics::RenderingState;
use crate::game::model::linemap::Linemap;

pub fn render_linemap_3d(linemap: &Linemap, rendering_state: &RenderingState, renderer: &Renderer) {
    for hit_details in cast_rays_linemap(linemap, &rendering_state) {
        render_hit_column(&hit_details, rendering_state, renderer);
    }
}
