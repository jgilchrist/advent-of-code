use aoc::prelude::*;
use utils::prelude::*;

pub struct Day03;

type Section = Set<char>;
type Rucksack = (Section, Section);

fn priority_of(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 64 + 26,
        _ => unreachable!(),
    }
}

impl AocSolution for Day03 {
    type Input = Vec<Rucksack>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (left, right) = l.split_at(l.len() / 2);
                (left.chars().collect(), right.chars().collect())
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(8252);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .map(|(left, right)| left & right)
            .map(|set| set.into_iter().next().unwrap())
            .map(priority_of)
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(2828);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .map(|(left, right)| left | right)
            .tuples()
            .map(|(b1, b2, b3)| {
                b1.intersection(&b2)
                    .copied()
                    .collect::<Set<char>>()
                    .intersection(&b3)
                    .copied()
                    .collect::<Set<char>>()
            })
            .map(|set| set.into_iter().next().unwrap())
            .map(priority_of)
            .sum::<u32>()
    }
}
