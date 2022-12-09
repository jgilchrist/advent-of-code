use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn move_in_direction(&self, dir: impl Into<Self>) -> Self {
        self.move_in_direction_by(dir, 1)
    }

    #[must_use]
    pub fn move_in_direction_by(&self, dir: impl Into<Self>, amount: u32) -> Self {
        *self + dir.into() * amount as i32
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

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
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

impl From<(u32, u32)> for Vec2 {
    fn from(val: (u32, u32)) -> Self {
        Self::new(val.0 as i32, val.1 as i32)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(val: (usize, usize)) -> Self {
        Self::new(val.0 as i32, val.1 as i32)
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
