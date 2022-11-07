use crate::{aoc::Solution, utils::inputs::lines, AocSolution};

use fancy_regex::Regex;

pub struct Day05;

impl AocSolution for Day05 {
    fn get_input() -> &'static str {
        include_str!("d05.in")
    }

    type Input = Vec<String>;
    fn process_input(input: &str) -> Self::Input {
        lines(input)
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(255);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        fn is_nice(line: &str) -> bool {
            let number_of_vowels = line.matches('a').count()
                + line.matches('e').count()
                + line.matches('i').count()
                + line.matches('o').count()
                + line.matches('u').count();

            let doesnt_match_disallowed_strings = !line.contains("ab")
                && !line.contains("cd")
                && !line.contains("pq")
                && !line.contains("xy");

            let double_letter = Regex::new(r"(.)\1").unwrap().find(line).unwrap().is_some();

            number_of_vowels >= 3 && doesnt_match_disallowed_strings && double_letter
        }

        input.iter().filter(|l| is_nice(l)).count()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(55);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        fn is_nice(line: &str) -> bool {
            let has_matching_pair = Regex::new(r"(..).*\1")
                .unwrap()
                .find(line)
                .unwrap()
                .is_some();
            let has_xyz = Regex::new(r"(.).\1").unwrap().find(line).unwrap().is_some();

            has_matching_pair && has_xyz
        }

        input.iter().filter(|l| is_nice(l)).count()
    }
}
