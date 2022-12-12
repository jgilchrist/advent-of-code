use prelude::*;
use utils::{
    geometry::d2::{grid::Grid, vecs::Vec2},
    search::djikstra,
};

pub struct Day12;

#[derive(Clone)]
pub struct Cell(char);

impl Cell {
    fn is_start(&self) -> bool {
        self.0 == 'S'
    }

    fn is_end(&self) -> bool {
        self.0 == 'E'
    }

    fn height(&self) -> u8 {
        match self.0 {
            'S' => b'a',
            'E' => b'z',
            other => other as u8,
        }
    }

    fn can_traverse_to(&self, other: &Self) -> bool {
        other.height() as i8 - self.height() as i8 <= 1
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        Self(c)
    }
}

fn generate_successors(grid: &Grid<Cell>, pos: Vec2) -> Vec<(Vec2, u32)> {
    grid.neighbors4(pos)
        .filter(|neighbor| {
            grid.at(pos)
                .unwrap()
                .can_traverse_to(grid.at(*neighbor).unwrap())
        })
        .map(|neighbor| (neighbor, 1))
        .collect_vec()
}

fn is_goal(grid: &Grid<Cell>, pos: Vec2) -> bool {
    grid.at(pos).unwrap().is_end()
}

impl AocSolution for Day12 {
    type Input = (Grid<Cell>, Vec2);
    fn process_input(input: &str) -> Self::Input {
        let grid = inputs::grid_of::<Cell>(input);
        let start = grid
            .iter_cells()
            .find(|(_, cell)| cell.is_start())
            .unwrap()
            .0;

        (grid, start)
    }

    const PART1_SOLUTION: Solution = solution(462);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        let (grid, start) = input;

        let path = djikstra::djikstra(
            start,
            |pos| generate_successors(grid, *pos),
            |pos| is_goal(grid, *pos),
        )
        .unwrap();

        path.len() - 1
    }

    const PART2_SOLUTION: Solution = solution(451);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let (grid, _) = input;

        let starts = grid.cells_matching(|cell| cell.0 == 'a');

        let path = djikstra::djikstra_from_n_states(
            &starts,
            |pos| generate_successors(grid, *pos),
            |pos| is_goal(grid, *pos),
        )
        .unwrap();

        path.len() - 1
    }
}
