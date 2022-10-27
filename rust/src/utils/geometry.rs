use super::vecs::Vec2;

#[derive(Debug)]
pub struct Square {
    top_left: Vec2,
    bottom_right: Vec2,
}

impl Square {
    pub fn new(top_left: Vec2, bottom_right: Vec2) -> Self {
        Square {
            top_left,
            bottom_right,
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.top_left.x <= point.x
            && point.x <= self.bottom_right.x
            && self.top_left.y <= point.y
            && point.y <= self.bottom_right.y
    }
}
