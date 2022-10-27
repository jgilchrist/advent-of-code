use crate::{aoc::Solution, AocSolution};

pub struct Day16;

impl AocSolution for Day16 {
    fn get_input() -> &'static str {
        include_str!("d16.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = &'static str;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("11100110111101110");
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = &'static str;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("10001101010000101");
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
