use aoc::prelude::*;


pub struct Day01;

fn number_of_positive_diffs(ns: &[i32]) -> usize {
    ns.array_windows()
        .map(|&[a, b]| b - a)
        .filter(|diff| *diff > 0)
        .count()
}

impl AocSolution for Day01 {
    type Input = Vec<i32>;
    fn process_input(input: &str) -> Self::Input {
        input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(1184);
    fn part1(input: &Self::Input) -> impl ToSolution {
        number_of_positive_diffs(input)
    }

    const PART2_SOLUTION: SolutionStatus = solution(1158);
    fn part2(input: &Self::Input) -> impl ToSolution {
        number_of_positive_diffs(
            &input
                .windows(3)
                .map(|w| w.iter().sum())
                .collect::<Vec<i32>>(),
        )
    }
}
