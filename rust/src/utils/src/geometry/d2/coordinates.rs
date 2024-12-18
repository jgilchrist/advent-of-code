use super::vecs::Vec2;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TurnDirection {
    Clockwise,
    Anticlockwise,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

// https://en.wikipedia.org/wiki/Compass_rose
// Cardinal + intercardinal directions are referred to as the 'principal winds'
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum PrincipalWinds {
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
    pub const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

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

impl PrincipalWinds {
    pub const ALL: [Self; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    pub const DIAGONALS: [Self; 4] = [
        Self::NorthEast,
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthWest,
    ];

    pub fn components(&self) -> Option<[CardinalDirection; 2]> {
        use CardinalDirection::*;

        Some(match self {
            Self::NorthEast => [North, East],
            Self::SouthEast => [South, East],
            Self::SouthWest => [South, West],
            Self::NorthWest => [North, West],
            Self::North | Self::East | Self::South | Self::West => return None,
        })
    }
}

impl From<CardinalDirection> for PrincipalWinds {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => Self::North,
            CardinalDirection::East => Self::East,
            CardinalDirection::South => Self::South,
            CardinalDirection::West => Self::West,
        }
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

impl TryFrom<Vec2> for CardinalDirection {
    type Error = ();

    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        use CardinalDirection::*;

        Ok(match (value.x, value.y) {
            (0, -1) => North,
            (0, 1) => South,
            (1, 0) => East,
            (-1, 0) => West,
            _ => return Err(()),
        })
    }
}

impl From<PrincipalWinds> for Vec2 {
    fn from(value: PrincipalWinds) -> Self {
        use PrincipalWinds::*;
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
