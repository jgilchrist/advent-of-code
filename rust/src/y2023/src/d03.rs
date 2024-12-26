use aoc::prelude::*;
use utils::geometry::d2::coordinates::CardinalDirection;
use utils::geometry::d2::grid::Grid;
use utils::geometry::d2::vecs::Vec2;
use utils::prelude::*;

pub struct Day03;

#[derive(Debug, PartialEq, Eq)]
pub enum SymbolType {
    Gear,
    Other,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Digit(char),
    Symbol(SymbolType),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        if c == '.' {
            return Self::Empty;
        }

        if c.is_ascii_digit() {
            return Self::Digit(c);
        }

        if c == '*' {
            return Self::Symbol(SymbolType::Gear);
        }

        Self::Symbol(SymbolType::Other)
    }
}

fn extract_number(g: &Grid<Cell>, seen_coords: &mut Set<Vec2>, start: Vec2) -> Option<u32> {
    let mut number: String = String::new();
    let Cell::Digit(d) = g.at(start).unwrap() else {
        panic!();
    };

    if seen_coords.contains(&start) {
        return None;
    }
    seen_coords.insert(start);

    number.insert(0, *d);

    let mut to_start = start;
    to_start = to_start.move_in_direction(CardinalDirection::West);

    while let Some(Cell::Digit(d)) = g.at(to_start) {
        if seen_coords.contains(&to_start) {
            return None;
        }
        seen_coords.insert(to_start);
        to_start = to_start.move_in_direction(CardinalDirection::West);

        number.insert(0, *d);
    }

    let mut to_end = start;
    to_end = to_end.move_in_direction(CardinalDirection::East);

    while let Some(Cell::Digit(d)) = g.at(to_end) {
        if seen_coords.contains(&to_end) {
            return None;
        }
        seen_coords.insert(to_end);
        to_end = to_end.move_in_direction(CardinalDirection::East);

        number.push(*d);
    }

    Some(number.parse::<u32>().unwrap())
}

fn get_symbol_coords(grid: &Grid<Cell>) -> Vec<Vec2> {
    grid.iter_cells()
        .filter(|(_, val)| matches!(val, Cell::Symbol(_)))
        .map(|(c, _)| c)
        .collect::<Vec<_>>()
}

fn get_numbers_next_to_symbols(grid: &Grid<Cell>) -> Vec<Vec<u32>> {
    let symbol_coords = get_symbol_coords(grid);

    symbol_coords
        .iter()
        .map(|&sym| {
            let (numbers, _) = grid
                .neighbors8(sym)
                .filter(|&c| matches!(grid.at(c).unwrap(), Cell::Digit(_)))
                .fold(
                    (Vec::<u32>::new(), Set::<Vec2>::new()),
                    |(mut numbers, mut seen_coords), coord| {
                        let Some(number) = extract_number(grid, &mut seen_coords, coord) else {
                            return (numbers, seen_coords);
                        };

                        numbers.push(number);
                        (numbers, seen_coords)
                    },
                );

            numbers
        })
        .collect::<Vec<_>>()
}

impl AocSolution for Day03 {
    type Input = Grid<Cell>;
    fn process_input(input: &str) -> Self::Input {
        inputs::grid_of::<Cell>(input)
    }

    const PART1_SOLUTION: SolutionStatus = solution(535235);
    fn part1(input: &Self::Input) -> impl ToSolution {
        get_numbers_next_to_symbols(input)
            .iter()
            .flatten()
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(79844424);
    fn part2(input: &Self::Input) -> impl ToSolution {
        get_numbers_next_to_symbols(input)
            .iter()
            .filter(|ns| ns.len() == 2)
            .map(|ns| ns[0] * ns[1])
            .sum::<u32>()
    }
}
