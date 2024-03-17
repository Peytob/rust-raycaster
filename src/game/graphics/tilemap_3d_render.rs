use sdl2::pixels::Color;
use crate::game::graphics::ray_caster::{cast_rays_tilemap, Hit};
use crate::game::graphics::renderer::Renderer;
use crate::game::graphics::RenderingState;
use crate::game::model::tilemap::Tilemap;

pub fn render_tilemap_3d(tilemap: &Tilemap, rendering_state: &RenderingState, renderer: &Renderer) {
    for hit_details in cast_rays_tilemap(tilemap, &rendering_state) {
        let ray = hit_details.ray();
        match hit_details.hit() {
            Hit::None => {
                renderer.render_column(&ray, hit_details.column(), hit_details.total_columns(), &Color::WHITE);
            }

            Hit::Wall { placed_tile } => {
                renderer.render_column(&ray, hit_details.column(), hit_details.total_columns(), &placed_tile.tile().color());
            }
        };
    }
}
