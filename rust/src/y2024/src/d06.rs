use aoc::prelude::*;
use utils::geometry::d2::coordinates::CardinalDirection;
use utils::geometry::d2::grid::Grid;
use utils::geometry::d2::vecs::Vec2;
use utils::prelude::*;

pub struct Day06;

#[derive(Eq, PartialEq)]
pub enum ParsedCell {
    Empty,
    Obstruction,
    InitialPosition,
}

impl From<char> for ParsedCell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Obstruction,
            '^' => Self::InitialPosition,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum Cell {
    Empty,
    Obstruction,
}

impl AocSolution for Day06 {
    type Input = (Grid<Cell>, Vec2, CardinalDirection);

    fn process_input(input: &str) -> Self::Input {
        let parsed_grid = inputs::grid_of::<ParsedCell>(input);

        let initial_position = parsed_grid
            .iter_cells()
            .find(|(_, c)| **c == ParsedCell::InitialPosition)
            .unwrap()
            .0;

        let initial_direction = CardinalDirection::North;

        let grid: Grid<Cell> = parsed_grid.map_cells(|_, c| match c {
            ParsedCell::Empty | ParsedCell::InitialPosition => Cell::Empty,
            ParsedCell::Obstruction => Cell::Obstruction,
        });

        (grid, initial_position, initial_direction)
    }

    const PART1_SOLUTION: SolutionStatus = solution(5242);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let (grid, mut position, mut direction) = input.clone();

        let mut seen_cells = HashSet::<Vec2>::new();

        loop {
            seen_cells.insert(position);
            let in_front = position.move_in_direction(direction);

            match grid.at(in_front).cloned() {
                // We've left the grid!
                None => break,

                // We've not left the grid - see what's in front of us
                Some(c) => match c {
                    Cell::Empty => position = in_front,
                    Cell::Obstruction => direction = direction.clockwise(),
                },
            }
        }

        seen_cells.len()
    }

    const PART2_SOLUTION: SolutionStatus = solution(1424);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let (grid, start_position, _) = input.clone();

        let mut position = start_position;
        let mut direction = CardinalDirection::North;
        let mut seen_cells = HashSet::<Vec2>::new();

        // First, run part 1 to get the path with no added obstructions
        loop {
            seen_cells.insert(position);
            let in_front = position.move_in_direction(direction);

            match grid.at(in_front).cloned() {
                // We've left the grid!
                None => break,

                // We've not left the grid - see what's in front of us
                Some(c) => match c {
                    Cell::Empty => position = in_front,
                    Cell::Obstruction => direction = direction.clockwise(),
                },
            }
        }

        // Create a set of new grids, each of which has a new obstacle on a cell that didn't have it before
        seen_cells
            .iter()
            // For each potential replacement position, we create a new grid with that one cell changed
            .map(|&replace_pos| {
                let mut new_grid = grid.clone();
                new_grid.replace_at(replace_pos, Cell::Obstruction);
                new_grid
            })
            .filter(|grid| is_patrol_loop(grid, start_position))
            .count()
    }
}

fn is_patrol_loop(grid: &Grid<Cell>, start: Vec2) -> bool {
    let mut seen_states = HashSet::<(Vec2, CardinalDirection)>::new();
    let mut position = start;
    let mut direction = CardinalDirection::North;

    loop {
        seen_states.insert((position, direction));
        let in_front = position.move_in_direction(direction);

        match grid.at(in_front).cloned() {
            // We've left the grid!
            None => return false,

            // We've not left the grid - see what's in front of us
            Some(c) => match c {
                Cell::Empty => {
                    position = in_front;

                    // If, after moving, we're in a state we've seen before, we're going to loop
                    if seen_states.contains(&(position, direction)) {
                        return true;
                    }
                }
                Cell::Obstruction => direction = direction.clockwise(),
            },
        }
    }
}
