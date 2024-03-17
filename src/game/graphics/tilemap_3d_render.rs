use crate::game::graphics::ray_caster::cast_rays_tilemap;
use crate::game::graphics::renderer::{render_hit_column, Renderer};
use crate::game::graphics::RenderingState;
use crate::game::model::tilemap::Tilemap;

pub fn render_tilemap_3d(tilemap: &Tilemap, rendering_state: &RenderingState, renderer: &Renderer) {
    for hit_details in cast_rays_tilemap(tilemap, &rendering_state) {
        render_hit_column(&hit_details, rendering_state, renderer);
    }
}
