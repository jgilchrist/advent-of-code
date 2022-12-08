#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

use itertools::Itertools;

use super::{
    coordinates::{CardinalDirection, ExtendedCardinalDirection},
    vecs::Vec2,
};

#[derive(Clone)]
pub struct Map<T> {
    size: u32,
    cells: Vec<T>,
}

impl<T> Map<T> {
    pub fn new(cells: Vec<T>) -> Self {
        let size = (cells.len() as f32).sqrt() as u32;
        Self { size, cells }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn at(&self, v: Vec2) -> Option<&T> {
        if v.x < 0 {
            unreachable!();
        }
        if v.y < 0 {
            unreachable!();
        }
        let idx: usize = v.y as usize * self.size as usize + v.x as usize;
        self.cells.get(idx)
    }

    pub fn neighbors4(&self, v: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        CardinalDirection::all()
            .map(move |dir| v.move_in_direction(dir))
            .filter(|coord| self.is_valid_coord(coord))
    }

    pub fn neighbors8(&self, v: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        ExtendedCardinalDirection::all()
            .map(move |dir| v.move_in_direction(dir))
            .filter(|coord| self.is_valid_coord(coord))
    }

    pub fn neighbor_cells4(&self, v: Vec2) -> impl Iterator<Item = &T> + '_ {
        self.neighbors4(v).map(|coord| self.at(coord).unwrap())
    }

    pub fn neighbor_cells8(&self, v: Vec2) -> impl Iterator<Item = &T> + '_ {
        self.neighbors8(v).map(|coord| self.at(coord).unwrap())
    }

    pub fn is_valid_coord(&self, v: &Vec2) -> bool {
        v.x >= 0 && v.x < self.size as i32 && v.y >= 0 && v.y < self.size as i32
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Vec2> + '_ {
        (0..self.size).flat_map(move |y| (0..self.size).map(move |x| Vec2::new(x as i32, y as i32)))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (Vec2, &T)> + '_ {
        self.iter_coords().map(|c| (c, self.at(c).unwrap()))
    }

    pub fn map_cells<X, F>(&self, f: F) -> Map<X>
    where
        F: Fn(Vec2, &T) -> X,
    {
        Map::new(
            self.iter_cells()
                .map(|(coord, value)| f(coord, value))
                .collect_vec(),
        )
    }

    pub fn raycast_coords(
        &self,
        from_coord: Vec2,
        direction: CardinalDirection,
    ) -> impl Iterator<Item = Vec2> + '_ {
        (1..self.size)
            .map(move |amount| from_coord.move_in_direction_by(direction, amount))
            .take_while(|coord| self.is_valid_coord(coord))
    }

    pub fn raycast_cells(
        &self,
        from_coord: Vec2,
        direction: CardinalDirection,
    ) -> impl Iterator<Item = &T> + '_ {
        self.raycast_coords(from_coord, direction)
            .map(|coord| self.at(coord).unwrap())
    }
}

impl<T> std::fmt::Debug for Map<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (coord, value) in self.iter_cells() {
            if coord.x == 0 {
                writeln!(f)?;
            }

            write!(f, "{value:?}")?;
        }

        writeln!(f)
    }
}
