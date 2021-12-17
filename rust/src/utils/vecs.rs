use std::ops::Add;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
