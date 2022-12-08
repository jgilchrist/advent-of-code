use super::vecs::Vec2;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TurnDirection {
    Clockwise,
    Anticlockwise,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn all() -> impl Iterator<Item = Self> {
        vec![Self::North, Self::East, Self::South, Self::West].into_iter()
    }

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
        use CardinalDirection::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    #[must_use]
    pub fn anticlockwise(&self) -> Self {
        use CardinalDirection::*;
        match *self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn as_vec(&self) -> Vec2 {
        use CardinalDirection::*;
        match *self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
        .into()
    }
}
