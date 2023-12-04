use aoc::prelude::*;
use utils::prelude::*;

pub struct Day04;

fn all_hashes(input: &str) -> impl Iterator<Item = String> + '_ {
    let natural_numbers = iterate(0, |&i| i + 1);
    let natural_numbers_as_strings = natural_numbers.map(|i| i.to_string());

    let infinite_challenge_input = std::iter::repeat(input);

    let inputs_with_numbers = infinite_challenge_input.zip(natural_numbers_as_strings);
    let inputs = inputs_with_numbers.map(|(a, b)| a.to_owned() + &b);

    inputs.map(|i| format!("{:x}", md5::compute(i)))
}

impl AocSolution for Day04 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.trim().to_owned()
    }

    const PART1_SOLUTION: SolutionStatus = solution(346386);
    fn part1(input: &Self::Input) -> impl ToSolution {
        for (i, hash) in all_hashes(input).enumerate() {
            if hash.starts_with("00000") {
                return i;
            }
        }

        unreachable!()
    }

    const PART2_SOLUTION: SolutionStatus = solution(9958218);
    fn part2(input: &Self::Input) -> impl ToSolution {
        for (i, hash) in all_hashes(input).enumerate() {
            if hash.starts_with("000000") {
                return i;
            }
        }

        unreachable!()
    }
}
