use prelude::*;

pub struct Day17;

fn matching_combinations(input: &[u32]) -> impl Iterator<Item = Vec<&u32>> {
    input
        .iter()
        .powerset()
        .collect_vec()
        .into_iter()
        .filter(|s| s.iter().copied().sum::<u32>() == 150u32)
}

impl AocSolution for Day17 {
    type Input = Vec<u32>;
    fn process_input(input: &str) -> Self::Input {
        inputs::positive_numbers(&input.lines().join(" "))
    }

    const PART1_SOLUTION: Solution = solution(1638);
    fn part1(input: &Self::Input) -> impl ToSolution {
        matching_combinations(input).count()
    }

    const PART2_SOLUTION: Solution = solution(17);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let (_, &number_of_combinations) = matching_combinations(input)
            .counts_by(|c| c.len())
            .iter()
            .min_by_key(|(number_of_continers, _)| *number_of_continers)
            .unwrap();

        number_of_combinations
    }
}
