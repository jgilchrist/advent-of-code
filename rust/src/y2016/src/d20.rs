use prelude::*;

pub struct Day20;

impl AocSolution for Day20 {
    fn get_input() -> &'static str {
        include_str!("d20.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: Solution = solution(19449262);
    const PART1_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part1(_input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }

    const PART2_SOLUTION: Solution = solution(119);
    const PART2_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part2(_input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}
