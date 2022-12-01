use prelude::*;

pub struct DayDAY;

impl AocSolution for DayDAY {
    fn get_input() -> &'static str {
        include_str!("dDAY.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.to_owned()
    }

    const PART1_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART1_SOLUTION: Solution = Solution::Unsolved;
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }

    const PART2_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART2_SOLUTION: Solution = Solution::Unsolved;
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}
