use crate::AocSolution;
use itertools::iterate;

pub struct Day04;

fn all_hashes(input: &str) -> impl Iterator<Item = String> + '_ {
    let natural_numbers = iterate(0, |&i| i + 1);
    let natural_numbers_as_strings = natural_numbers.map(|i| i.to_string());

    let infinite_challenge_input = std::iter::repeat(input);

    let inputs_with_numbers = infinite_challenge_input.zip(natural_numbers_as_strings);
    let inputs = inputs_with_numbers.map(|(a, b)| a.to_string() + &b);

    let hashes = inputs.map(|i| format!("{:x}", md5::compute(i)));

    hashes
}

impl AocSolution for Day04 {
    type Input = String;
    type Output = usize;

    fn get_input() -> &'static str {
        include_str!("d04.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.trim().to_string()
    }

    const PART1_SOLUTION: Option<Self::Output> = Some(346386);
    fn part1(input: &Self::Input) -> Self::Output {
        for (i, hash) in all_hashes(input).enumerate() {
            if hash.starts_with("00000") {
                return i;
            }
        }

        unreachable!()
    }

    const PART2_SOLUTION: Option<Self::Output> = Some(9958218);
    fn part2(input: &Self::Input) -> Self::Output {
        for (i, hash) in all_hashes(input).enumerate() {
            if hash.starts_with("000000") {
                return i;
            }
        }

        unreachable!()
    }
}

