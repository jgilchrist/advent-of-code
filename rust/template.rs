use crate::{AocSolution, aoc::Solution};

pub struct DayDAY;

impl AocSolution for DayDAY {
    type Input = String;
    type Output = &'static str;

    fn get_input() -> &'static str {
        include_str!("dDAY.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .collect()
    }

    const PART1_SOLUTION: Solution<Self::Output> = Solution::UnsolvedWithKnownAnswerFromPython();
    fn part1(input: &Self::Input) -> Self::Output {
        todo!()
    }

    const PART2_SOLUTION: Solution<Self::Output> = Solution::UnsolvedWithKnownAnswerFromPython;
    fn part2(input: &Self::Input) -> Self::Output {
        todo!()
    }
}
