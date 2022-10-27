use crate::{aoc::Solution, AocSolution};

pub struct Day01;

impl AocSolution for Day01 {
    fn get_input() -> &'static str {
        include_str!("d01.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(55776);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        todo!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> =
        Solution::UnsolvedWithKnownAnswerFromPython(223162626);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        todo!()
    }
}
