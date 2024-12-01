#![allow(clippy::similar_names)]

use aoc::prelude::*;
use utils::inputs::positive_numbers;
use utils::iters::transpose;
use utils::prelude::*;

pub struct Day01;

impl AocSolution for Day01 {
    type Input = (Vec<u32>, Vec<u32>);
    fn process_input(input: &str) -> Self::Input {
        let vecs: Vec<Vec<u32>> = input.lines().map(positive_numbers).collect::<_>();

        let vecs = transpose(&vecs);

        let vec1 = vecs[0].clone().into_iter().sorted().collect::<Vec<_>>();
        let vec2 = vecs[1].clone().into_iter().sorted().collect::<Vec<_>>();

        (vec1, vec2)
    }

    const PART1_SOLUTION: SolutionStatus = solution(2430334);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let (vec1, vec2) = input;

        vec1.iter()
            .zip(vec2.iter())
            .map(|(&a, &b)| a.abs_diff(b))
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(28786472);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let (vec1, vec2) = input;

        let counts = vec2.iter().counts();

        vec1.iter()
            .map(|i| i * *counts.get(i).unwrap_or(&0) as u32)
            .sum::<u32>()
    }
}
