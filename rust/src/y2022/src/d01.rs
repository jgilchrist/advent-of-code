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

    const PART1_SOLUTION: SolutionStatus = solution(66616);
    fn part1(input: &Self::Input) -> impl ToSolution {
        *input.first().unwrap()
    }

    const PART2_SOLUTION: SolutionStatus = solution(199172);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input.iter().take(3).sum::<u32>()
    }
}
