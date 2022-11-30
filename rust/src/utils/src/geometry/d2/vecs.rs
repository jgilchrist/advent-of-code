use std::ops::{Add, AddAssign, Mul};

use super::coordinates::CardinalDirection;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn move_in_direction(&self, dir: CardinalDirection) -> Self {
        self.move_in_direction_by(dir, 1)
    }

    #[must_use]
    pub fn move_in_direction_by(&self, dir: CardinalDirection, amount: i32) -> Self {
        *self + dir.as_vec() * amount
    }

    pub fn distance_from(&self, rhs: impl Into<Self>) -> usize {
        let r = rhs.into();

        ((self.x - r.x).abs() + (self.y - r.y).abs())
            .try_into()
            .unwrap()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(val: (i32, i32)) -> Self {
        Self::new(val.0, val.1)
    }
}
