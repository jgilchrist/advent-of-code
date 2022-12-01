use prelude::*;

pub struct Day21;

impl AocSolution for Day21 {
    fn get_input() -> &'static str {
        include_str!("d21.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = &'static str;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("gbhafcde");
    fn part1(_input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = &'static str;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython("bcfaegdh");
    fn part2(_input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
