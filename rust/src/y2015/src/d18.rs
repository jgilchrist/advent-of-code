use prelude::*;
use utils::geometry::d2::{map::Map, vecs::Vec2};

pub struct Day18;

#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
    On,
    Off,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::On => '#',
                Self::Off => '.',
            }
        )
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::On,
            '.' => Self::Off,
            _ => unreachable!(),
        }
    }
}

fn evolve_using<F>(initial_state: &Map<Cell>, iters: u32, f: F) -> Map<Cell>
where
    F: Fn(&Map<Cell>, Vec2, &Cell) -> Cell,
{
    let mut state = initial_state.clone();

    for _ in 0..iters {
        state = state.map_cells(|coord, cell| f(&state, coord, cell));
    }

    state
}

fn game_of_life_rule(map: &Map<Cell>, coord: Vec2, cell: &Cell) -> Cell {
    let lit_neighbors = map
        .neighbor_cells8(coord)
        .filter(|&c| *c == Cell::On)
        .count();

    match cell {
        Cell::On => {
            if lit_neighbors == 2 || lit_neighbors == 3 {
                Cell::On
            } else {
                Cell::Off
            }
        }
        Cell::Off => {
            if lit_neighbors == 3 {
                Cell::On
            } else {
                Cell::Off
            }
        }
    }
}

fn count_lit_cells(map: &Map<Cell>) -> usize {
    map.iter_cells()
        .filter(|&(_, value)| *value == Cell::On)
        .count()
}

impl AocSolution for Day18 {
    type Input = Map<Cell>;
    fn process_input(input: &str) -> Self::Input {
        let cells: Vec<Cell> = input.lines().join("").chars().map(|c| c.into()).collect();

        Map::new(cells)
    }

    const PART1_SOLUTION: Solution = solution(768);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        count_lit_cells(&evolve_using(input, 100, |map, coord, cell| {
            game_of_life_rule(map, coord, cell)
        }))
    }

    const PART2_SOLUTION: Solution = solution(781);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let map_size = input.size() - 1;

        let corners: HashSet<Vec2> =
            vec![(0, 0), (map_size, 0), (0, map_size), (map_size, map_size)]
                .into_iter()
                .map(|c| c.into())
                .collect();

        count_lit_cells(&evolve_using(input, 100, |map, coord, cell| {
            if corners.contains(&coord) {
                Cell::On
            } else {
                game_of_life_rule(map, coord, cell)
            }
        }))
    }
}
