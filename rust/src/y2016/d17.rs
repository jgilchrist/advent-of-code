use crate::{aoc::Solution, AocSolution};

pub struct Day17;

impl AocSolution for Day17 {
    fn get_input() -> &'static str {
        include_str!("d17.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = &'static str;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("DDURRLRRDD");
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(436);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
