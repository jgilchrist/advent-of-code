use prelude::*;

pub struct DayDAY;

impl AocSolution for DayDAY {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.to_owned()
    }

    fn part1(input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }

    fn part2(input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}
