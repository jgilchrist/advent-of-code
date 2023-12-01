use prelude::*;

pub struct Day05;

impl AocSolution for Day05 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python("c6697b55");
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = solution_from_python("8c35d1ab");
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
