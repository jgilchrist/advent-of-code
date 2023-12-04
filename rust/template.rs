use aoc::prelude::*;
use utils::prelude::*;

pub struct DayDAY;

impl AocSolution for DayDAY {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.to_owned()
    }

    const PART1_SOLUTION: SolutionStatus = SolutionStatus::Wip;
    fn part1(input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = SolutionStatus::Wip;
    fn part2(input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
