use crate::AocSolution;

use fancy_regex::Regex;

pub struct Day05;

impl AocSolution for Day05 {
    type Input = Vec<String>;
    type Output = usize;

    fn get_input() -> &'static str {
        include_str!("d05.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.lines().map(|l| l.to_string()).collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = Some(255);
    fn part1(input: &Self::Input) -> Self::Output {
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

    const PART2_SOLUTION: Option<Self::Output> = Some(55);
    fn part2(input: &Self::Input) -> Self::Output {
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
