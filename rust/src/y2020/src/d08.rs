use prelude::*;

pub struct Day08;

impl AocSolution for Day08 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: Solution = solution(1262);
    const PART1_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: Solution = solution(1643);
    const PART2_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
