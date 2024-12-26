use aoc::prelude::*;
use utils::prelude::*;

pub struct Day11;

type State = Map<u64, usize>;
fn next_state(state: &State) -> State {
    let mut new_state = Map::new();

    for (&value, &n) in state {
        let value_as_str = value.to_string();

        if value == 0 {
            *new_state.entry(value + 1).or_insert(0) += n;
        } else if value_as_str.len() % 2 == 0 {
            let digits_a = value_as_str[..value_as_str.len() / 2]
                .parse::<u64>()
                .unwrap();
            let digits_b = value_as_str[value_as_str.len() / 2..]
                .parse::<u64>()
                .unwrap();

            *new_state.entry(digits_a).or_insert(0) += n;
            *new_state.entry(digits_b).or_insert(0) += n;
        } else {
            *new_state.entry(value * 2024).or_insert(0) += n;
        }
    }

    new_state
}

impl AocSolution for Day11 {
    type Input = State;
    fn process_input(input: &str) -> Self::Input {
        let mut map = Map::new();

        for n in inputs::positive_numbers(input) {
            *map.entry(u64::from(n)).or_insert(0) += 1;
        }

        map
    }

    const PART1_SOLUTION: SolutionStatus = solution(199753);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let mut state = input.clone();

        for _ in 0..25 {
            state = next_state(&state);
        }

        state.values().sum::<usize>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(239413123020116i64);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut state = input.clone();

        for _ in 0..75 {
            state = next_state(&state);
        }

        state.values().sum::<usize>()
    }
}
