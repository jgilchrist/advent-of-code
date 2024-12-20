use aoc::prelude::*;
use utils::prelude::*;

pub struct Day07;

fn get_minimum_cost_by(i: &[u64], cost_fn: impl Fn(u64, u64) -> u64) -> u64 {
    let positions_to_costs: HashMap<u64, u64> = (*i.iter().min().unwrap()
        ..=*i.iter().max().unwrap())
        .map(|pos| (pos, i.iter().map(|crab_pos| cost_fn(pos, *crab_pos)).sum()))
        .collect();

    *positions_to_costs.iter().min_by_key(|&x| *x.1).unwrap().1
}

#[allow(clippy::as_conversions)]
fn abs_difference(a: u64, b: u64) -> u64 {
    (a as i64 - b as i64).unsigned_abs()
}

impl AocSolution for Day07 {
    type Input = Vec<u64>;
    fn process_input(input: &str) -> Self::Input {
        inputs::comma_separated::<u64>(input)
    }

    const PART1_SOLUTION: SolutionStatus = solution(328262);
    fn part1(input: &Self::Input) -> impl ToSolution {
        get_minimum_cost_by(input, abs_difference)
    }

    const PART2_SOLUTION: SolutionStatus = solution(90040997);
    fn part2(input: &Self::Input) -> impl ToSolution {
        get_minimum_cost_by(input, |pos, crab_pos| {
            let diff = abs_difference(pos, crab_pos);
            (diff * (diff + 1)) / 2
        })
    }
}
