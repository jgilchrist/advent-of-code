use aoc::prelude::*;

pub struct Day16;

impl AocSolution for Day16 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python("11100110111101110");
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = solution_from_python("10001101010000101");
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
