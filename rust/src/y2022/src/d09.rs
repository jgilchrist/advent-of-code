use prelude::*;
use utils::geometry::d2::{coordinates::CardinalDirection, vecs::Vec2};

pub struct Day09;

impl AocSolution for Day09 {
    type Input = Vec<CardinalDirection>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .flat_map(|l| {
                let (dir, amount) = l.split_once(' ').unwrap();

                let dir = match dir {
                    "U" => CardinalDirection::North,
                    "D" => CardinalDirection::South,
                    "L" => CardinalDirection::West,
                    "R" => CardinalDirection::East,
                    _ => unreachable!(),
                };

                let amount = amount.parse::<usize>().unwrap();

                std::iter::repeat(dir).take(amount)
            })
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(6057);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        simulate_chain(input, 2)
    }

    const PART2_SOLUTION: Solution = solution(2514);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        simulate_chain(input, 10)
    }
}

fn simulate_chain(instructions: &[CardinalDirection], len: usize) -> usize {
    let mut visited_coords: HashSet<Vec2> = HashSet::new();
    let mut sections = vec![Vec2::new(0, 0); len];

    for &direction in instructions {
        // Move the head
        sections[0] = sections[0].move_in_direction(direction);

        for idx in 0..len - 1 {
            sections[idx + 1] = move_tail(sections[idx], sections[idx + 1]);
        }

        visited_coords.insert(*sections.last().unwrap());
    }

    visited_coords.len()
}

fn move_tail(head: Vec2, tail: Vec2) -> Vec2 {
    if head.chessboard_distance_from(tail) <= 1 {
        tail
    } else {
        tail + (head - tail).sign()
    }
}
