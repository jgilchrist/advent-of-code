use crate::AocSolution;

pub struct DayN;

impl AocSolution<N> for DayN {
    type Input = String;
    type Output = &'static str;

    fn get_input() -> &'static str {
        include_str!("")
    }

    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = None;
    fn part1(i: &Self::Input) -> Self::Output {
        todo!()
    }

    const PART2_SOLUTION: Option<Self::Output> = None;
    fn part2(i: &Self::Input) -> Self::Output {
        todo!()
    }
}