use prelude::*;
use utils::geometry::d2::coordinates::CardinalDirection;
use utils::geometry::d2::grid::Grid;
use utils::geometry::d2::vecs::Vec2;

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
            return Cell::Empty;
        }

        if c.is_ascii_digit() {
            return Cell::Digit(c);
        }

        if c == '*' {
            return Cell::Symbol(SymbolType::Gear);
        }

        Cell::Symbol(SymbolType::Other)
    }
}

fn extract_number(g: &Grid<Cell>, seen_coords: &mut HashSet<Vec2>, start: Vec2) -> Option<u32> {
    let mut number: String = String::new();
    let Cell::Digit(d) = g.at(start).unwrap() else {
        panic!();
    };

    if seen_coords.contains(&start) {
        return None;
    }
    seen_coords.insert(start);

    number.insert(0, *d);

    let mut to_start = start.clone();
    to_start = to_start.move_in_direction(CardinalDirection::West);

    while let Some(Cell::Digit(d)) = g.at(to_start) {
        if seen_coords.contains(&to_start) {
            return None;
        }
        seen_coords.insert(to_start);
        to_start = to_start.move_in_direction(CardinalDirection::West);

        number.insert(0, *d);
    }

    let mut to_end = start.clone();
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

impl AocSolution for Day03 {
    type Input = Grid<Cell>;
    fn process_input(input: &str) -> Self::Input {
        inputs::grid_of::<Cell>(input)
    }

    const PART1_SOLUTION: SolutionStatus = solution(535235);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let symbol_coords = input
            .iter_cells()
            .filter(|(_, val)| matches!(val, Cell::Symbol(_)))
            .map(|(c, val)| c)
            .collect_vec();

        symbol_coords
            .iter()
            .flat_map(|&sym| {
                let (numbers, _) = input
                    .neighbors8(sym)
                    .filter(|&c| matches!(input.at(c).unwrap(), Cell::Digit(_)))
                    .fold(
                        (Vec::<u32>::new(), HashSet::<Vec2>::new()),
                        |(mut numbers, mut seen_coords), coord| {
                            let Some(number) = extract_number(input, &mut seen_coords, coord)
                            else {
                                return (numbers, seen_coords);
                            };

                            numbers.push(number);
                            (numbers, seen_coords)
                        },
                    );

                numbers
            })
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(79844424);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let symbol_coords = input
            .iter_cells()
            .filter(|(_, val)| matches!(val, Cell::Symbol(_)))
            .map(|(c, val)| c)
            .collect_vec();

        symbol_coords
            .iter()
            .map(|&sym| {
                let (numbers, _) = input
                    .neighbors8(sym)
                    .filter(|&c| matches!(input.at(c).unwrap(), Cell::Digit(_)))
                    .fold(
                        (Vec::<u32>::new(), HashSet::<Vec2>::new()),
                        |(mut numbers, mut seen_coords), coord| {
                            let Some(number) = extract_number(input, &mut seen_coords, coord)
                            else {
                                return (numbers, seen_coords);
                            };

                            numbers.push(number);
                            (numbers, seen_coords)
                        },
                    );

                numbers
            })
            .filter(|ns| ns.len() == 2)
            .map(|ns| ns[0] * ns[1])
            .sum::<u32>()
    }
}
