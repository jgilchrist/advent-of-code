use prelude::*;

pub struct Day01;

fn number_of_positive_diffs(ns: &[i32]) -> usize {
    ns.windows(2)
        .map(|w| w[1] - w[0])
        .filter(|diff| *diff > 0)
        .count()
}

impl AocSolution for Day01 {
    fn get_input() -> &'static str {
        include_str!("d01.in")
    }

    type Input = Vec<i32>;
    fn process_input(input: &str) -> Self::Input {
        input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(1184);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        number_of_positive_diffs(input)
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(1158);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        number_of_positive_diffs(
            &input
                .windows(3)
                .map(|w| w.iter().sum())
                .collect::<Vec<i32>>(),
        )
    }
}
