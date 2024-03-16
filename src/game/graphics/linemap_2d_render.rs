use num_traits::zero;
use sdl2::pixels::Color;

use crate::game::graphics::renderer::Renderer;
use crate::game::graphics::RenderingState;
use crate::game::model::linemap::Linemap;

pub fn render_linemap_2d(linemap: &Linemap, rendering_state: &RenderingState, renderer: &Renderer) {
    renderer.render_2d_rect(&Color::WHITE, &zero(), &linemap.sizes());

    for line in linemap.lines() {
        renderer.render_2d_line(line.from(), line.to(), line.color())
    }
}
