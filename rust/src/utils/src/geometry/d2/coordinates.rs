use super::vecs::Vec2;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TurnDirection {
    Clockwise,
    Anticlockwise,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CompassHeading {
    North,
    East,
    South,
    West,
}

impl CompassHeading {
    #[must_use]
    pub fn turn(&self, direction: TurnDirection) -> Self {
        use TurnDirection::*;
        match direction {
            Clockwise => self.clockwise(),
            Anticlockwise => self.anticlockwise(),
        }
    }

    #[must_use]
    pub fn clockwise(&self) -> Self {
        use CompassHeading::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    #[must_use]
    pub fn anticlockwise(&self) -> Self {
        use CompassHeading::*;
        match *self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn as_vec(&self) -> Vec2 {
        use CompassHeading::*;
        match *self {
            North => Vec2::new(0, -1),
            South => Vec2::new(0, 1),
            East => Vec2::new(1, 0),
            West => Vec2::new(-1, 0),
        }
    }
}
