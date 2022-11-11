use itertools::Itertools;

use crate::{aoc::Solution, AocSolution};

pub struct Day10;

fn compress_number(number: &str) -> String {
    let mut chars = number.chars().collect_vec();
    chars.push(' ');

    let mut result = String::new();
    let mut count = 1;

    for window in chars.windows(2) {
        let &[last_char, current_char] = window else { unreachable!() };

        if current_char == last_char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push_str(&last_char.to_string());
            count = 1;
        }
    }

    result.to_string()
}

impl AocSolution for Day10 {
    fn get_input() -> &'static str {
        include_str!("d10.in")
    }

    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.trim().to_owned()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(252594);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        let mut s = input.clone();

        for _ in 0..40 {
            s = compress_number(&s);
        }

        s.len()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(3579328);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let mut s = input.clone();

        for _ in 0..50 {
            s = compress_number(&s);
        }

        s.len()
    }
}
