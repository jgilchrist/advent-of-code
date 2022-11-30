use super::vecs::Vec2;

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
