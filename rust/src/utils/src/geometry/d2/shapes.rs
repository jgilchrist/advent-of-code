use super::vecs::Vec2;
use crate::hash::{Set, SetBuilder};

#[derive(Debug)]
pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn points(&self) -> Set<Vec2> {
        let line_direction = (self.end - self.start).sign();
        let mut points: Set<Vec2> = Set::new();

        let mut current_point = self.start;
        points.insert(current_point);

        while current_point != self.end {
            current_point += line_direction;
            points.insert(current_point);
        }

        points
    }
}

#[derive(Debug)]
pub struct Square {
    top_left: Vec2,
    bottom_right: Vec2,
}

impl Square {
    pub fn new(top_left: impl Into<Vec2>, bottom_right: impl Into<Vec2>) -> Self {
        Self {
            top_left: top_left.into(),
            bottom_right: bottom_right.into(),
        }
    }

    pub fn contains(&self, point: impl Into<Vec2>) -> bool {
        let p = point.into();

        self.top_left.x <= p.x
            && p.x <= self.bottom_right.x
            && self.top_left.y <= p.y
            && p.y <= self.bottom_right.y
    }
}
