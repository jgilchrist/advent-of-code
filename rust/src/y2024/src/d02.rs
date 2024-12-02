use aoc::prelude::*;
use utils::prelude::*;

pub struct Day02;

fn is_safe(line: &[u32]) -> bool {
    (line.array_windows().all(|[a, b]| a > b) || line.array_windows().all(|[a, b]| a < b))
        && line
            .array_windows()
            .all(|[a, b]| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3)
}

impl AocSolution for Day02 {
    type Input = Vec<Vec<u32>>;
    fn process_input(input: &str) -> Self::Input {
        input.lines().map(inputs::positive_numbers).collect::<_>()
    }

    const PART1_SOLUTION: SolutionStatus = solution(341);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input.iter().filter(|v| is_safe(v)).count()
    }

    const PART2_SOLUTION: SolutionStatus = solution(404);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .filter(|&v| {
                // For each vector, create a list of vectors with one element removed
                (0..v.len())
                    .map(|idx| {
                        let mut v = v.clone();
                        v.remove(idx);
                        v
                    })
                    // And check if the resulting vector is safe
                    .any(|v| is_safe(&v))
            })
            .count()
    }
}
