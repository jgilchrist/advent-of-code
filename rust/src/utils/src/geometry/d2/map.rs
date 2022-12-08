#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

use super::{coordinates::CardinalDirection, vecs::Vec2};

#[derive(Debug)]
pub struct Map<T> {
    size: u32,
    cells: Vec<T>,
}

impl<T> Map<T> {
    pub fn new(cells: Vec<T>) -> Self {
        let size = (cells.len() as f32).sqrt() as u32;
        Self { size, cells }
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

    pub fn is_valid_coord(&self, v: &Vec2) -> bool {
        v.x >= 0 && v.x < self.size as i32 && v.y >= 0 && v.y < self.size as i32
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Vec2> + '_ {
        (0..self.size).flat_map(move |y| (0..self.size).map(move |x| Vec2::new(x as i32, y as i32)))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (Vec2, &T)> + '_ {
        self.iter_coords().map(|c| (c, self.at(c).unwrap()))
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
