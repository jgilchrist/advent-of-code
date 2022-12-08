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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ExtendedCardinalDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
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
}

impl ExtendedCardinalDirection {
    pub fn all() -> impl Iterator<Item = Self> {
        vec![
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
        ]
        .into_iter()
    }
}

impl From<CardinalDirection> for Vec2 {
    fn from(value: CardinalDirection) -> Self {
        use CardinalDirection::*;
        match value {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
        .into()
    }
}

impl From<ExtendedCardinalDirection> for Vec2 {
    fn from(value: ExtendedCardinalDirection) -> Self {
        use ExtendedCardinalDirection::*;
        match value {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
            NorthEast => (1, -1),
            NorthWest => (-1, -1),
            SouthEast => (1, 1),
            SouthWest => (-1, 1),
        }
        .into()
    }
}
