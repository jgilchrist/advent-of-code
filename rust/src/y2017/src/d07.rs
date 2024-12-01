use aoc::prelude::*;

pub struct Day07;

impl AocSolution for Day07 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python("mwzaxaj");
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = SolutionStatus::Unsolved;
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
