use crate::AocSolution;

pub struct Day01;

fn number_of_positive_diffs(ns: &[i32]) -> usize {
    ns.windows(2)
        .map(|w| w[1] - w[0])
        .filter(|diff| *diff > 0)
        .count()
}

impl AocSolution<1> for Day01 {
    type Input = Vec<i32>;
    type Output = usize;

    fn get_input() -> &'static str {
        include_str!("d01.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = Some(1184);
    fn part1(i: &Self::Input) -> Self::Output {
        number_of_positive_diffs(i)
    }

    const PART2_SOLUTION: Option<Self::Output> = Some(1158);
    fn part2(i: &Self::Input) -> Self::Output {
        number_of_positive_diffs(&i.windows(3).map(|w| w.iter().sum()).collect::<Vec<i32>>())
    }
}
