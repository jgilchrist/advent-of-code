use aoc::{AocSolution, Solution};
use itertools::Itertools;
use utils::inputs::separated_by;

pub struct Day01;

impl AocSolution for Day01 {
    fn get_input() -> &'static str {
        include_str!("d01.in")
    }

    type Input = Vec<u32>;
    fn process_input(input: &str) -> Self::Input {
        let mut calories = input
            .split("\n\n")
            .map(|group| separated_by::<u32>(group, "\n"))
            .map(|group| group.iter().sum())
            .collect_vec();

        calories.sort_unstable();
        calories.reverse();
        calories
    }

    type Part1Output = u32;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(66616);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        *input.first().unwrap()
    }

    type Part2Output = u32;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(199172);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        input.iter().take(3).sum()
    }
}
