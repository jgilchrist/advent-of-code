use prelude::*;

pub struct Day11;

impl AocSolution for Day11 {
    fn get_input() -> &'static str {
        include_str!("d11.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(31);
    fn part1(_input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(55);
    fn part2(_input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
