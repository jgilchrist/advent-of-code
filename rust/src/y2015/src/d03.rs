use prelude::*;

use utils::geometry::d2::{coordinates::CardinalDirection, vecs::Vec2};

pub struct Day03;

impl AocSolution for Day03 {
    type Input = Vec<CardinalDirection>;
    fn process_input(input: &str) -> Self::Input {
        input
            .trim()
            .chars()
            .map(|c| match c {
                '^' => CardinalDirection::North,
                'v' => CardinalDirection::South,
                '<' => CardinalDirection::West,
                '>' => CardinalDirection::East,
                _ => unreachable!(),
            })
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(2565);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        let mut pos = Vec2 { x: 0, y: 0 };
        let mut visited: HashSet<Vec2> = HashSet::from([pos]);

        for direction in input {
            pos = pos.move_in_direction(*direction);
            visited.insert(pos);
        }

        visited.len()
    }

    const PART2_SOLUTION: Solution = solution(2639);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let mut santa = Vec2 { x: 0, y: 0 };
        let mut robo_santa = Vec2 { x: 0, y: 0 };

        let mut visited: HashSet<Vec2> = HashSet::from([santa, robo_santa]);

        let santas_instructions = input.iter().step_by(2).collect_vec();
        let robo_santas_instructions = input.iter().skip(1).step_by(2).collect_vec();

        for direction in santas_instructions {
            santa = santa.move_in_direction(*direction);
            visited.insert(santa);
        }

        for direction in robo_santas_instructions {
            robo_santa = robo_santa.move_in_direction(*direction);
            visited.insert(robo_santa);
        }

        visited.len()
    }
}
