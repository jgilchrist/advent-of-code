use prelude::*;
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
        .map(|i| position.move_in_direction_by(new_heading, i.try_into().unwrap()))
        .collect_vec();

    let new_position = *visited_positions.last().unwrap();

    let state = (new_position, new_heading);
    (state, visited_positions)
}

impl AocSolution for Day01 {
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
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(252);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let start = (0, 0);
        let mut state: State = (start.into(), CardinalDirection::North);

        for instruction in input.iter() {
            (state, _) = run_instruction(&state, instruction);
        }

        let (final_position, _) = state;
        final_position.manhattan_distance_from(start)
    }

    const PART2_SOLUTION: Solution = solution(143);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut visited_locations: HashSet<Vec2> = HashSet::new();

        let start: Vec2 = (0, 0).into();
        let mut state = (start, CardinalDirection::North);

        for instruction in input.iter() {
            let (new_state, locs) = run_instruction(&state, instruction);
            state = new_state;

            for loc in locs {
                if visited_locations.contains(&loc) {
                    return loc.manhattan_distance_from(start);
                }

                visited_locations.insert(loc);
            }
        }

        unreachable!()
    }
}
