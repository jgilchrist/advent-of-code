use prelude::*;

pub struct Day01;

impl AocSolution for Day01 {
    type Input = Vec<u32>;
    fn process_input(input: &str) -> Self::Input {
        let mut calories = input
            .split("\n\n")
            .map(|group| inputs::separated_by::<u32>(group, "\n"))
            .map(|group| group.iter().sum())
            .collect_vec();

        calories.sort_unstable();
        calories.reverse();
        calories
    }

    fn part1(input: &Self::Input) -> impl Into<Solution> {
        *input.first().unwrap()
    }

    fn part2(input: &Self::Input) -> impl Into<Solution> {
        input.iter().take(3).sum::<u32>()
    }
}

aoc::solutions!(Day01, 2022, 01, 66616, 199172);
