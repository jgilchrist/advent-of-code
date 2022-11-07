use fancy_regex::Regex;

use crate::{aoc::Solution, AocSolution, utils::inputs::lines};

pub struct Day08;

fn regex_replace(s: &str, regex: &str, replace_with: &str) -> String {
    Regex::new(regex).unwrap().replace_all(&s, replace_with).to_string()
}

fn count_chars_in_line(line: &str) -> usize {
    let line = &line[1 .. line.len() - 1];

    let line = regex_replace(&line, r#"\\\\"#, ".");
    let line = regex_replace(&line, r#"\\\""#, ".");
    let line = regex_replace(&line, r#"\\x[0-9A-Fa-f]{2}"#, ".");

    line.len()
}

fn count_expanded_chars_in_line(line: &str) -> usize {
    let line = regex_replace(&line, r#"""#, "..");
    let line = regex_replace(&line, r#"\\"#, "..");
    let line = r#"""#.to_string() + &line + r#"""#;

    line.len()
}

impl AocSolution for Day08 {
    fn get_input() -> &'static str {
        include_str!("d08.in")
    }

    type Input = Vec<String>;
    fn process_input(input: &str) -> Self::Input {
        lines(input)
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(1342);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        let total_len: usize = input.iter().map(|l| l.len()).sum();
        let string_len: usize = input.iter().map(|l| count_chars_in_line(l)).sum();

        total_len - string_len
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(2074);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let total_len: usize = input.iter().map(|l| l.len()).sum();
        let expanded_len: usize = input.iter().map(|l| count_expanded_chars_in_line(l)).sum();

        expanded_len - total_len
    }
}
