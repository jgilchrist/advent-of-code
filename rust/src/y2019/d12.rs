use crate::{aoc::Solution, AocSolution};

pub struct Day12;

impl AocSolution for Day12 {
    fn get_input() -> &'static str {
        include_str!("d12.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(12053);
    fn part1(_input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(320380285873116);
    fn part2(_input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
