use aoc::prelude::*;
use utils::geometry::d2::coordinates::PrincipalWinds;
use utils::geometry::d2::grid::{Grid, GridPrintChars};
use utils::geometry::d2::vecs::Vec2;
use utils::prelude::*;

pub struct Day15;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Robot,
    Wall,
    Box,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Robot,
            '#' => Self::Wall,
            'O' => Self::Box,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

impl GridPrintChars for Cell {
    fn as_char(&self) -> char {
        match self {
            Self::Robot => '@',
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Empty => '.',
        }
    }
}

impl AocSolution for Day15 {
    type Input = (Grid<Cell>, Vec2, Vec<PrincipalWinds>);
    fn process_input(input: &str) -> Self::Input {
        let (grid, moves) = input.split_once("\n\n").unwrap();

        let grid = inputs::grid_of::<Cell>(grid);

        let moves = moves
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                '^' => PrincipalWinds::North,
                'v' => PrincipalWinds::South,
                '<' => PrincipalWinds::West,
                '>' => PrincipalWinds::East,
                c => {
                    println!("{c} oh no");
                    unreachable!()
                }
            })
            .collect();

        let start_pos = *grid.cells_matching(|c| *c == Cell::Robot).first().unwrap();

        (grid, start_pos, moves)
    }

    const PART1_SOLUTION: SolutionStatus = SolutionStatus::Wip;
    fn part1(input: &Self::Input) -> impl ToSolution {
        let (mut grid, start_pos, moves) = input.clone();

        let mut pos = start_pos;

        for direction in moves {
            let potentially_pushed_boxes = grid
                .raycast_cells(pos, direction)
                .take_while_inclusive(|(_, s)| **s == Cell::Box)
                .map(|(pos, s)| (pos, *s))
                .collect::<Vec<_>>();

            let (last_cell_pos, last_cell) = potentially_pushed_boxes.last().unwrap();
            match *last_cell {
                Cell::Empty => {
                    // The robot moves to the position of the first box
                    grid.set_at(pos, Cell::Empty);
                    grid.set_at(pos.move_in_direction(direction), Cell::Robot);
                    pos = pos.move_in_direction(direction);

                    // We might not have moved any boxes but only stepped into the next cell.
                    if potentially_pushed_boxes.len() > 1 {
                        // The last empty cell becomes a box
                        grid.set_at(*last_cell_pos, Cell::Box);
                    }
                }
                // If the boxes are up against a wall, we won't modify anything as they can't be pushed
                Cell::Wall => continue,
                Cell::Box | Cell::Robot => unreachable!(),
            }
        }

        grid.cells_matching(|c| *c == Cell::Box)
            .iter()
            .map(|pos| pos.y * 100 + pos.x)
            .sum::<i32>() as usize
    }

    const PART2_SOLUTION: SolutionStatus = SolutionStatus::Wip;
    fn part2(input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
