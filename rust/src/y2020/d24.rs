use crate::{AocSolution, aoc::Solution};

pub struct Day24;

impl AocSolution for Day24 {
    type Input = String;

    fn get_input() -> &'static str {
        include_str!("d24.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::UnsolvedWithKnownAnswerFromPython(523);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::UnsolvedWithKnownAnswerFromPython(4225);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
