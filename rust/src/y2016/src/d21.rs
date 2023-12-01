use prelude::*;

pub struct Day21;

impl AocSolution for Day21 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python("gbhafcde");
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = solution_from_python("bcfaegdh");
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
