use crate::AocSolution;

pub struct Day01;

fn number_of_positive_diffs(ns: &Vec<i32>) -> usize {
    ns.windows(2)
        .map(|w| w[1] - w[0])
        .filter(|diff| *diff > 0)
        .collect::<Vec<i32>>()
        .len()
}

impl AocSolution for Day01 {
    type Input = Vec<i32>;
    type Output = usize;

    const YEAR: u32 = 2021;
    const DAY: u32 = 01;

    fn process_input(&self, input: &str) -> Self::Input {
        input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
    }

    fn part1(&self, i: &Self::Input) -> Self::Output {
        number_of_positive_diffs(i)
    }

    fn part2(&self, i: &Self::Input) -> Self::Output {
        number_of_positive_diffs(&i.windows(3).map(|w| w.iter().sum()).collect::<Vec<i32>>())
    }
}
