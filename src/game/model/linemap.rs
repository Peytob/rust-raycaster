use glm::{abs, Vec2};
use num_traits::{zero, Zero};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::game::model::repository::Resource;
use crate::game::model::ResourceId;

const EPSILON: f32 = 0.005;

pub struct Line {
    color: Color,
    from: Vec2,
    to: Vec2,
    k: f32,
    b: f32
}

impl Line {
    pub fn new(color: Color, from: Vec2, to: Vec2) -> Self {
        let x_diff = from.x - to.x;

        let mut k = 0f32;
        let mut b = from.y;

        if !x_diff.is_zero() {
            k = (from.y - to.y) / x_diff;
            b = from.y - k * from.x
        }

        Self { color, from, to, k, b }
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
    pub fn b(&self) -> f32 {
        self.b
    }
    pub fn k(&self) -> f32 {
        self.k
    }

    pub fn compute_at_x(&self, x: f32) -> f32 {
        return self.k * x + self.b;
    }

    pub fn contains_x(&self, x: f32) -> bool {
        return contains_in_range(self.from.x, self.to.x, x);
    }

    pub fn contains_y(&self, y: f32) -> bool {
        return contains_in_range(self.from.y, self.to.y, y);
    }

    pub fn find_intersection(first: &Line, second: &Line) -> Option<Vec2> {

        let intersection =
        if is_parallel_y(first) {
            let intersection_x = first.from.x;
            Vec2::new(intersection_x, second.compute_at_x(intersection_x))
        } else if is_parallel_y(second) {
            let intersection_x = second.from.x;
            Vec2::new(intersection_x, first.compute_at_x(intersection_x))
        } else
        {
            let intersection_x = (second.b - first.b) / (first.k - second.k);
            Vec2::new(intersection_x, first.compute_at_x(intersection_x))
        };

        // let intersection = if is_parallel_y(first) {
        //     Vec2::new(first.from.x, second.compute_at_x(first.from.x))
        // } else if is_parallel_y(second) {
        //     Vec2::new(second.from.x, first.compute_at_x(second.from.x))
        // } else {
        //     let intersection_x = (second.b - first.b) / (first.k - second.k);
        //     Vec2::new(intersection_x, first.compute_at_x(intersection_x))
        // };

        // if first.contains_x(intersection.x) && second.contains_x(intersection.x) {
        //     return Some(intersection);
        // }
        //
        // return None;

        if  !first.contains_x(intersection.x) ||
            !first.contains_y(intersection.y) ||
            !second.contains_x(intersection.x) ||
            !second.contains_y(intersection.y)  {
            return None
        }

        return Some(intersection);
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

fn contains_in_range(from: f32, to: f32, value: f32) -> bool {
    let (min_val, max_val) = if from < to {
        (from, to)
    } else {
        (to, from)
    };

    return min_val - EPSILON <= value && value <= max_val + EPSILON;
}

fn is_parallel_y(line: &Line) -> bool {
    abs(line.from.x - line.to.x) < EPSILON
}
