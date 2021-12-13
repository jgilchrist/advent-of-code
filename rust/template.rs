use crate::AocSolution;

pub struct DayN;

impl AocSolution for DayN {
    type Input = String;
    type Output = &'static str;

    const DAY: u32 = 0;

    fn get_input() -> Self::Input {
        include_str!("src/y2021/d01.in")
            .lines()
            .map(|l| l.chars().collect())
            .collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = None;
    fn part1(i: &Self::Input) -> Self::Output {
        ""
    }

    const PART2_SOLUTION: Option<Self::Output> = None;
    fn part2(i: &Self::Input) -> Self::Output {
        ""
    }
}