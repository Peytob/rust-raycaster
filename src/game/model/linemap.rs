use glm::Vec2;
use num_traits::zero;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

pub struct Line {
    color: Color,
    from: Vec2,
    to: Vec2
}

impl Line {
    pub fn new(color: Color, from: Vec2, to: Vec2) -> Self {
        Self { color, from, to }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn from(&self) -> &Vec2 {
        &self.from
    }
    pub fn to(&self) -> &Vec2 {
        &self.to
    }
}

pub struct Linemap {
    id: ResourceId,
    lines: Vec<Line>,
    sizes: Vec2
}

impl Linemap {
    pub fn new(id: ResourceId) -> Self {
        Self {
            id,
            lines: vec![],
            sizes: zero()
        }
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }
    pub fn sizes(&self) -> &Vec2 {
        &self.sizes
    }

    pub fn add_line(&mut self, color: Color, from: Vec2, to: Vec2) -> &mut Linemap {
        let line = Line::new(color, from, to);

        self.sizes.x = line.from.x.max(self.sizes.x).max(line.to.x);
        self.sizes.y = line.from.y.max(self.sizes.y).max(line.to.y);

        self.lines.push(line);
        return self;
    }

    pub fn add_rect(&mut self, color: Color, rect: Rect) -> &mut Linemap {
        self.add_line(color.clone(), point_to_vec2(rect.top_left()), point_to_vec2(rect.top_right()));
        self.add_line(color.clone(), point_to_vec2(rect.top_right()), point_to_vec2(rect.bottom_right()));
        self.add_line(color.clone(), point_to_vec2(rect.bottom_right()), point_to_vec2(rect.bottom_left()));
        self.add_line(color, point_to_vec2(rect.bottom_left()), point_to_vec2(rect.top_left()));

        return self;
    }
}

impl Resource for Linemap {
    fn id(&self) -> ResourceId {
        self.id
    }
}

fn point_to_vec2(point: Point) -> Vec2 {
    Vec2::new(point.x as f32, point.y as f32)
}
