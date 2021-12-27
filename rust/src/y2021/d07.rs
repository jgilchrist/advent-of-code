use std::collections::HashMap;

use crate::{utils::inputs::comma_separated_integers, AocSolution};

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

impl AocSolution<7> for Day07 {
    type Input = Vec<u64>;
    type Output = u64;

    fn get_input() -> &'static str {
        include_str!("d07.in")
    }

    fn process_input(input: &str) -> Self::Input {
        comma_separated_integers(input)
            .iter()
            .map(|x| *x as u64)
            .collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = Some(328262);
    fn part1(i: &Self::Input) -> Self::Output {
        get_minimum_cost_by(i, |pos, crab_pos| {
            (pos as i64 - crab_pos as i64).unsigned_abs()
        })
    }

    const PART2_SOLUTION: Option<Self::Output> = Some(90040997);
    fn part2(i: &Self::Input) -> Self::Output {
        get_minimum_cost_by(i, |pos, crab_pos| {
            let diff = (pos as i64 - crab_pos as i64).unsigned_abs();
            (diff * (diff + 1)) / 2
        })
    }
}
