use prelude::*;
use std::hash::Hash;

pub struct Day06;

fn index_of_first_distinct_sequence<T>(ts: &[T], window_size: usize) -> usize
where
    T: Hash + Eq + std::fmt::Debug,
{
    ts.windows(window_size)
        .enumerate()
        .find(|(_, window)| window.iter().all_unique())
        .map(|(idx, _)| idx + window_size)
        .unwrap()
}

impl AocSolution for Day06 {
    type Input = Vec<char>;
    fn process_input(input: &str) -> Self::Input {
        input.chars().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(1766);
    fn part1(input: &Self::Input) -> impl ToSolution {
        index_of_first_distinct_sequence(input, 4)
    }

    const PART2_SOLUTION: SolutionStatus = solution(2383);
    fn part2(input: &Self::Input) -> impl ToSolution {
        index_of_first_distinct_sequence(input, 14)
    }
}
