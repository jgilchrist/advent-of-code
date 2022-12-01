use prelude::*;

pub struct Day16;

impl AocSolution for Day16 {
    fn get_input() -> &'static str {
        include_str!("d16.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: Solution = solution(21071);
    const PART1_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part1(_input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }

    const PART2_SOLUTION: Solution = solution(3429967441937u64);
    const PART2_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part2(_input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}
