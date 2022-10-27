use crate::{AocSolution, aoc::Solution};

pub struct Day08;

impl AocSolution for Day08 {
    type Input = String;

    fn get_input() -> &'static str {
        include_str!("d08.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::UnsolvedWithKnownAnswerFromPython(5966);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::UnsolvedWithKnownAnswerFromPython(6347);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
