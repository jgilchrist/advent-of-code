use std::collections::HashMap;

use crate::{aoc::Solution, utils::inputs::comma_separated_integers, AocSolution};

pub struct Day07;

fn get_minimum_cost_by<TFn>(i: &[u64], cost_fn: TFn) -> u64
where
    TFn: Fn(u64, u64) -> u64,
{
    let positions_to_costs: HashMap<u64, u64> = (*i.iter().min().unwrap()
        ..=*i.iter().max().unwrap())
        .map(|pos| (pos, i.iter().map(|crab_pos| cost_fn(pos, *crab_pos)).sum()))
        .collect();

    *positions_to_costs.iter().min_by_key(|&x| *x.1).unwrap().1
}

impl AocSolution for Day07 {
    fn get_input() -> &'static str {
        include_str!("d07.in")
    }

    type Input = Vec<u64>;
    fn process_input(input: &str) -> Self::Input {
        comma_separated_integers(input)
            .iter()
            .map(|x| *x as u64)
            .collect()
    }

    type Part1Output = u64;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solution(328262);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        get_minimum_cost_by(input, |pos, crab_pos| {
            (pos as i64 - crab_pos as i64).unsigned_abs()
        })
    }

    type Part2Output = u64;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solution(90040997);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        get_minimum_cost_by(input, |pos, crab_pos| {
            let diff = (pos as i64 - crab_pos as i64).unsigned_abs();
            (diff * (diff + 1)) / 2
        })
    }
}
