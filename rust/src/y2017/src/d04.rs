use aoc::prelude::*;

pub struct Day04;

impl AocSolution for Day04 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python(337);
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = solution_from_python(231);
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
