use aoc::prelude::*;

pub struct Day02;

impl AocSolution for Day02 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python(43074);
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = solution_from_python(280);
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
