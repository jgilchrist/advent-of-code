use std::collections::HashSet;

use aoc::{AocSolution, Solution};
use itertools::Itertools;
use utils::geometry::d2::{
    coordinates::{CardinalDirection, TurnDirection},
    vecs::Vec2,
};

pub struct Instruction {
    direction: TurnDirection,
    amount: u8,
}

pub struct Day01;

type State = (Vec2, CardinalDirection);

fn run_instruction(state: &State, instruction: &Instruction) -> (State, Vec<Vec2>) {
    let &(position, heading) = state;
    let new_heading = heading.turn(instruction.direction);

    let visited_positions = (1..=instruction.amount)
        .map(|i| position + new_heading.as_vec() * i.try_into().unwrap())
        .collect_vec();

    let new_position = *visited_positions.last().unwrap();

    let state = (new_position, new_heading);
    (state, visited_positions)
}

impl AocSolution for Day01 {
    fn get_input() -> &'static str {
        include_str!("d01.in")
    }

    type Input = Vec<Instruction>;
    fn process_input(input: &str) -> Self::Input {
        input
            .trim()
            .split(", ")
            .map(|s| Instruction {
                direction: match s.chars().next().unwrap() {
                    'L' => TurnDirection::Anticlockwise,
                    'R' => TurnDirection::Clockwise,
                    _ => unreachable!(),
                },
                amount: s[1..].parse::<u8>().unwrap(),
            })
            .collect_vec()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(252);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        let start = Vec2::new(0, 0);
        let mut state = (start, CardinalDirection::North);

        for instruction in input.iter() {
            (state, _) = run_instruction(&state, instruction);
        }

        let (final_position, _) = state;
        final_position.distance_from(start)
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(143);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let mut visited_locations: HashSet<Vec2> = HashSet::new();

        let start = Vec2::new(0, 0);
        let mut state = (start, CardinalDirection::North);

        for instruction in input.iter() {
            let (new_state, locs) = run_instruction(&state, instruction);
            state = new_state;

            for loc in locs {
                if visited_locations.contains(&loc) {
                    return loc.distance_from(start);
                }

                visited_locations.insert(loc);
            }
        }

        unreachable!()
    }
}
